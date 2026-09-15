use std::sync::{Arc, Mutex};

use ocs_plugin_api::host::{
    acadrust::{entities::Text, types::Vector3, EntityType},
    BuiltinPlugin, CommandStep, HostApi, InteractiveCommand,
};
use ocs_plugin_api::manifest::{ApiVersion, PluginManifest};
use ocs_plugin_api::ribbon::{CadModule, IconKind, ModuleEvent, RibbonGroup, RibbonItem, ToolDef};

const COMMAND: &str = "PNUM";
const TEXT_HEIGHT: f64 = 2.5;
const OFFSET: f64 = 1.25;

static MANIFEST: PluginManifest = PluginManifest {
    id: "opencad.point_numbering",
    name: "Point Numbering",
    version: env!("CARGO_PKG_VERSION"),
    description: "Place incrementing labels by clicking points in the drawing.",
    api_version: ApiVersion::CURRENT,
    ribbon_order: 60,
    xdata_apps: &[],
    command_prefixes: &[COMMAND],
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Settings {
    next: i64,
    increment: i64,
    prefix: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            next: 1,
            increment: 1,
            prefix: String::new(),
        }
    }
}

impl Settings {
    fn label(&self) -> String {
        format!("{}{}", self.prefix, self.next)
    }

    fn apply_command(&mut self, command: &str) -> Result<(), &'static str> {
        let args: Vec<_> = command.split_whitespace().skip(1).collect();
        if args.is_empty() {
            return Ok(());
        }
        if args.len() > 3 {
            return Err("Usage: PNUM [start] [increment] [prefix]");
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
}

struct PointNumberingModule;

impl CadModule for PointNumberingModule {
    fn id(&self) -> &'static str {
        MANIFEST.id
    }
    fn title(&self) -> &'static str {
        "Numbering"
    }

    fn ribbon_groups(&self) -> &[RibbonGroup] {
        static GROUPS: std::sync::OnceLock<Vec<RibbonGroup>> = std::sync::OnceLock::new();
        GROUPS.get_or_init(|| {
            vec![RibbonGroup {
                title: "Point labels",
                tools: vec![RibbonItem::LargeTool(ToolDef {
                    id: COMMAND,
                    label: "Number points",
                    // A glyph uses the host ribbon's existing icon styling and theme colors.
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
                .starts_with(&format!("{COMMAND} "))
        {
            return false;
        }

        let mut settings = self.settings.lock().expect("settings lock poisoned");
        if let Err(error) = settings.apply_command(command) {
            host.push_error(error);
            return true;
        }
        let message = format!(
            "Point numbering: next {}, increment {}, prefix '{}'. Click a point; Enter or Esc finishes. Configure: PNUM [start] [increment] [prefix].",
            settings.next, settings.increment, settings.prefix
        );
        drop(settings);
        host.push_info(&message);
        host.start_interactive(Box::new(NumberPoints {
            settings: Arc::clone(&self.settings),
        }));
        true
    }
}

struct NumberPoints {
    settings: Arc<Mutex<Settings>>,
}

impl InteractiveCommand for NumberPoints {
    fn prompt(&self) -> String {
        "Select label point (Enter or Esc to finish):".to_string()
    }

    fn on_point(&mut self, point: [f64; 3]) -> CommandStep {
        let mut settings = self.settings.lock().expect("settings lock poisoned");
        let label = settings.label();
        settings.next += settings.increment;
        drop(settings);

        let text = Text::with_value(
            label,
            Vector3::new(point[0] + OFFSET, point[1] + OFFSET, point[2]),
        )
        .with_height(TEXT_HEIGHT);
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
        settings.apply_command("PNUM 10 5 P-").unwrap();
        assert_eq!(settings.label(), "P-10");
        assert_eq!(settings.increment, 5);
    }
}
