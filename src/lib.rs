use std::sync::{Arc, Mutex};

#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::process::Command;

use ocs_plugin_api::host::{
    acadrust::{entities::Text, types::Vector3, EntityType},
    BuiltinPlugin, CommandStep, HostApi, InteractiveCommand,
};
use ocs_plugin_api::manifest::{ApiVersion, PluginManifest};
use ocs_plugin_api::ribbon::{CadModule, IconKind, ModuleEvent, RibbonGroup, RibbonItem, ToolDef};

const COMMAND: &str = "PNUM";

static MANIFEST: PluginManifest = PluginManifest {
    id: "opencad.point_numbering",
    name: "xfTools",
    version: env!("CARGO_PKG_VERSION"),
    description: "Place incrementing labels by clicking points in the drawing.",
    api_version: ApiVersion::CURRENT,
    ribbon_order: 60,
    xdata_apps: &[],
    command_prefixes: &[COMMAND],
};

#[derive(Debug, Clone)]
struct Settings {
    next: i64,
    increment: i64,
    prefix: String,
    text_height: f64,
    offset: f64,
    style: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            next: 1,
            increment: 1,
            prefix: String::new(),
            text_height: 2.5,
            offset: 1.25,
            style: "Standard".to_string(),
        }
    }
}

impl Settings {
    fn label(&self) -> String {
        format!("{}{}", self.prefix, self.next)
    }

    fn apply_command(&mut self, command: &str) -> Result<(), &'static str> {
        let args: Vec<_> = command
            .split_once(':')
            .filter(|(name, _)| name.eq_ignore_ascii_case(COMMAND))
            .map(|(_, tail)| tail.split(',').collect())
            .unwrap_or_else(|| command.split_whitespace().skip(1).collect());
        if args.is_empty() {
            return Ok(());
        }
        if args.len() > 3 || args[0].is_empty() {
            return Err("Usage: PNUM:start,increment,prefix (for example PNUM:1,1,P-)");
        }
        self.next = args[0].parse().map_err(|_| "Start must be an integer")?;
        if let Some(increment) = args.get(1) {
            self.increment = increment
                .parse()
                .map_err(|_| "Increment must be an integer")?;
        }
        if let Some(prefix) = args.get(2) {
            self.prefix = prefix.trim_matches('"').to_string();
        }
        Ok(())
    }

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    fn fields(&self) -> [String; 6] {
        [
            self.next.to_string(),
            self.increment.to_string(),
            self.prefix.clone(),
            self.text_height.to_string(),
            self.offset.to_string(),
            self.style.clone(),
        ]
    }

    #[cfg(any(target_os = "windows", target_os = "macos", test))]
    fn from_fields(fields: Vec<String>) -> Result<Self, String> {
        if fields.len() != 6 {
            return Err("The settings window returned incomplete values.".to_string());
        }
        let next = fields[0]
            .trim()
            .parse()
            .map_err(|_| "Start number must be an integer.".to_string())?;
        let increment = fields[1]
            .trim()
            .parse()
            .map_err(|_| "Increment must be an integer.".to_string())?;
        let text_height: f64 = fields[3]
            .trim()
            .replace(',', ".")
            .parse()
            .map_err(|_| "Text height must be a positive number.".to_string())?;
        let offset: f64 = fields[4]
            .trim()
            .replace(',', ".")
            .parse()
            .map_err(|_| "Offset must be a non-negative number.".to_string())?;
        if !text_height.is_finite() || text_height <= 0.0 {
            return Err("Text height must be a positive number.".to_string());
        }
        if !offset.is_finite() || offset < 0.0 {
            return Err("Offset must be a non-negative number.".to_string());
        }
        if fields[5].trim().is_empty() {
            return Err("Text style cannot be empty.".to_string());
        }
        Ok(Self {
            next,
            increment,
            prefix: fields[2].clone(),
            text_height,
            offset,
            style: fields[5].trim().to_string(),
        })
    }
}

#[derive(Clone, Copy)]
enum ConfigurationState {
    Pending,
    Ready,
    Cancelled,
}

#[cfg(target_os = "windows")]
fn show_settings(settings: Settings) -> Result<Option<Settings>, String> {
    let values = settings.fields().join("\t");
    let script = r#"
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
$v = $env:XFTOOLS_VALUES -split "`t", 6
$form = New-Object System.Windows.Forms.Form
$form.Text = 'xfTools - Point numbering'; $form.ClientSize = New-Object System.Drawing.Size(390, 285)
$form.StartPosition = 'CenterScreen'; $form.FormBorderStyle = 'FixedDialog'; $form.MaximizeBox = $false
function Field($label, $value, $y) {
  $l = New-Object System.Windows.Forms.Label; $l.Text = $label; $l.Location = New-Object System.Drawing.Point(16,$y); $l.Size = New-Object System.Drawing.Size(145,22)
  $t = New-Object System.Windows.Forms.TextBox; $t.Text = $value; $t.Location = New-Object System.Drawing.Point(170,($y-2)); $t.Size = New-Object System.Drawing.Size(200,22)
  $form.Controls.AddRange(@($l,$t)); return $t
}
$start = Field 'Start number' $v[0] 18; $increment = Field 'Increment' $v[1] 52; $prefix = Field 'Prefix' $v[2] 86
$height = Field 'Text height' $v[3] 120; $offset = Field 'Upper-right offset' $v[4] 154; $style = Field 'Text style' $v[5] 188
$cancel = New-Object System.Windows.Forms.Button; $cancel.Text='Cancel'; $cancel.Location=New-Object System.Drawing.Point(210,235); $cancel.DialogResult=[System.Windows.Forms.DialogResult]::Cancel
$ok = New-Object System.Windows.Forms.Button; $ok.Text='Start'; $ok.Location=New-Object System.Drawing.Point(295,235); $ok.DialogResult=[System.Windows.Forms.DialogResult]::OK
$form.AcceptButton=$ok; $form.CancelButton=$cancel; $form.Controls.AddRange(@($cancel,$ok))
if ($form.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) { Write-Output ($start.Text + "`t" + $increment.Text + "`t" + $prefix.Text + "`t" + $height.Text + "`t" + $offset.Text + "`t" + $style.Text) }
"#;
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ])
        .env("XFTOOLS_VALUES", values)
        .output()
        .map_err(|error| format!("Could not open settings window: {error}"))?;
    if !output.status.success() {
        return Ok(None);
    }
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if text.is_empty() {
        return Ok(None);
    }
    Settings::from_fields(text.split('\t').map(str::to_owned).collect()).map(Some)
}

#[cfg(target_os = "macos")]
fn show_settings(settings: Settings) -> Result<Option<Settings>, String> {
    let values = settings.fields();
    let script = r#"ObjC.import('Cocoa');

function field(value, y) {
  const input = $.NSTextField.alloc.initWithFrame($.NSMakeRect(160, y, 210, 24));
  input.setStringValue($(value));
  return input;
}

function label(text, y) {
  const output = $.NSTextField.alloc.initWithFrame($.NSMakeRect(0, y, 150, 24));
  output.setStringValue($(text));
  output.setBezeled(false);
  output.setDrawsBackground(false);
  output.setEditable(false);
  output.setSelectable(false);
  return output;
}

function run(argv) {
  const view = $.NSView.alloc.initWithFrame($.NSMakeRect(0, 0, 370, 210));
  const names = ['Start number', 'Increment', 'Prefix', 'Text height', 'Upper-right offset', 'Text style'];
  const inputs = [];
  for (let index = 0; index < names.length; index++) {
    const y = 180 - index * 30;
    const input = field(argv[index], y);
    view.addSubview(label(names[index], y));
    view.addSubview(input);
    inputs.push(input);
  }

  const alert = $.NSAlert.alloc.init;
  alert.setMessageText($('xfTools — Point numbering'));
  alert.setInformativeText($('Configure the labels before selecting points.'));
  alert.setAccessoryView(view);
  alert.addButtonWithTitle($('Start'));
  alert.addButtonWithTitle($('Cancel'));
  if (alert.runModal() != $.NSAlertFirstButtonReturn) return;

  console.log(inputs.map(input => ObjC.unwrap(input.stringValue)).join('\t'));
}"#;
    let output = Command::new("osascript")
        .args(["-l", "JavaScript", "-e", script])
        .args(values)
        .output()
        .map_err(|error| format!("Could not open settings window: {error}"))?;
    if !output.status.success() {
        return Ok(None);
    }
    Settings::from_fields(
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .split('\t')
            .map(str::to_owned)
            .collect(),
    )
    .map(Some)
}

#[cfg(target_os = "linux")]
fn show_settings(settings: Settings) -> Result<Option<Settings>, String> {
    // The AppImage does not bundle a portable native form toolkit. Keep Linux
    // functional with its existing inline settings rather than adding a system dependency.
    Ok(Some(settings))
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
fn show_settings(_settings: Settings) -> Result<Option<Settings>, String> {
    Err("This platform has no settings window.".to_string())
}

struct PointNumberingModule;
impl CadModule for PointNumberingModule {
    fn id(&self) -> &'static str {
        MANIFEST.id
    }
    fn title(&self) -> &'static str {
        "xfTools"
    }
    fn ribbon_groups(&self) -> &[RibbonGroup] {
        static GROUPS: std::sync::OnceLock<Vec<RibbonGroup>> = std::sync::OnceLock::new();
        GROUPS.get_or_init(|| {
            vec![RibbonGroup {
                title: "Point labels",
                tools: vec![RibbonItem::LargeTool(ToolDef {
                    id: COMMAND,
                    label: "Number points",
                    icon: IconKind::Glyph("1,2…"),
                    event: ModuleEvent::Command(COMMAND.to_string()),
                })],
            }]
        })
    }
}

struct PointNumberingPlugin {
    settings: Arc<Mutex<Settings>>,
}
impl PointNumberingPlugin {
    fn new() -> Self {
        Self {
            settings: Arc::new(Mutex::new(Settings::default())),
        }
    }
}

impl BuiltinPlugin for PointNumberingPlugin {
    fn manifest(&self) -> &'static PluginManifest {
        &MANIFEST
    }
    fn ribbon(&self) -> Box<dyn CadModule> {
        Box::new(PointNumberingModule)
    }
    fn dispatch(&self, host: &mut dyn HostApi, command: &str) -> bool {
        if !command.eq_ignore_ascii_case(COMMAND)
            && !command
                .to_ascii_uppercase()
                .starts_with(&format!("{COMMAND}:"))
            && !command
                .to_ascii_uppercase()
                .starts_with(&format!("{COMMAND} "))
        {
            return false;
        }
        let initial = {
            let mut settings = self.settings.lock().expect("settings lock poisoned");
            if let Err(error) = settings.apply_command(command) {
                host.push_error(error);
                return true;
            }
            settings.clone()
        };
        // Dialog is a separate OS process, so dispatch returns before the host timeout.
        let configuration = Arc::new(Mutex::new(ConfigurationState::Pending));
        let settings_for_dialog = Arc::clone(&self.settings);
        let configuration_for_dialog = Arc::clone(&configuration);
        std::thread::spawn(move || {
            let state = match show_settings(initial) {
                Ok(Some(selected)) => {
                    *settings_for_dialog.lock().expect("settings lock poisoned") = selected;
                    ConfigurationState::Ready
                }
                _ => ConfigurationState::Cancelled,
            };
            *configuration_for_dialog
                .lock()
                .expect("configuration lock poisoned") = state;
        });
        host.push_info("Set the numbering options in the xfTools window.");
        host.start_interactive(Box::new(NumberPoints {
            settings: Arc::clone(&self.settings),
            configuration,
        }));
        true
    }
}

struct NumberPoints {
    settings: Arc<Mutex<Settings>>,
    configuration: Arc<Mutex<ConfigurationState>>,
}
impl InteractiveCommand for NumberPoints {
    fn prompt(&self) -> String {
        match *self
            .configuration
            .lock()
            .expect("configuration lock poisoned")
        {
            ConfigurationState::Pending => "Configure options in the xfTools window...".to_string(),
            ConfigurationState::Ready => "Select label point (Enter or Esc to finish):".to_string(),
            ConfigurationState::Cancelled => {
                "Configuration cancelled. Press Enter or Esc.".to_string()
            }
        }
    }
    fn on_point(&mut self, point: [f64; 3]) -> CommandStep {
        match *self
            .configuration
            .lock()
            .expect("configuration lock poisoned")
        {
            ConfigurationState::Pending => return CommandStep::NeedPoint,
            ConfigurationState::Cancelled => return CommandStep::Done,
            ConfigurationState::Ready => {}
        }
        let mut settings = self.settings.lock().expect("settings lock poisoned");
        let label = settings.label();
        settings.next += settings.increment;
        let mut text = Text::with_value(
            label,
            Vector3::new(
                point[0] + settings.offset,
                point[1] + settings.offset,
                point[2],
            ),
        )
        .with_height(settings.text_height);
        text.style = settings.style.clone();
        CommandStep::Commit(EntityType::Text(text))
    }
    fn on_enter(&mut self) -> CommandStep {
        CommandStep::Done
    }
}

ocs_plugin_api::export_plugin!(PointNumberingPlugin::new());

#[cfg(test)]
mod tests {
    use super::Settings;
    #[test]
    fn command_sets_numbering_values() {
        let mut settings = Settings::default();
        settings.apply_command("PNUM:10,5,P-").unwrap();
        assert_eq!(settings.label(), "P-10");
        assert_eq!(settings.increment, 5);
    }
    #[test]
    fn dialog_values_set_text_properties() {
        let settings = Settings::from_fields(vec![
            "10".into(),
            "2".into(),
            "P-".into(),
            "3.5".into(),
            "1.0".into(),
            "Notes".into(),
        ])
        .unwrap();
        assert_eq!(settings.text_height, 3.5);
        assert_eq!(settings.style, "Notes");
    }
}
