# Quick guide — Point Numbering

> **Experimental:** save a copy of important DWG/DXF files and verify the generated labels before production use.

## What it does

The plugin places numeric text labels in a drawing. Once started, click a vertex or an empty location: a label is placed slightly above and to the right of the click. The next value is kept while OpenCADStudio remains open.

## First installation on Windows

1. Open the [latest release](https://github.com/franzo-ux/opencad-point-numbering/releases/latest).
2. Download **both** files:
   - `opencad.point_numbering-windows-x86_64.dll`
   - `plugin.toml`
3. Paste this path into File Explorer’s address bar:

   ```text
   %APPDATA%\OpenCADStudio\plugins\opencad.point_numbering\
   ```

4. Create the folder if it does not exist.
5. Copy both files into it.
6. Restart OpenCADStudio.

After restarting, the **xfTools** ribbon tab includes a **Number points** button with a `1,2…` icon.

## Standard numbering

1. Open a drawing.
2. Select **xfTools → Number points**, or enter `PNUM` in the command line.
3. Click points in the required order.
4. Press **Enter** or **Esc** to finish.

The first run starts at `1`, increments by `1`, and has no prefix.

## Configure or restart numbering

Before clicking points, enter:

```text
PNUM [start-number] [increment] [prefix]
```

| Goal | Command | Result |
| --- | --- | --- |
| Continue | `PNUM` | Keeps the session’s next value |
| Restart at 1 | `PNUM 1` | Restarts at 1, preserving the current increment and prefix |
| Add `P-` prefix | `PNUM 1 1 P-` | `P-1`, `P-2`, `P-3`… |
| Count by tens | `PNUM 100 10 PT-` | `PT-100`, `PT-110`, `PT-120`… |
| Count down | `PNUM 10 -1 N-` | `N-10`, `N-9`, `N-8`… |
| Clear prefix | `PNUM 1 1 ""` | `1`, `2`, `3`… |

Prefixes cannot contain spaces.

## Label appearance

- Labels are `TEXT` entities, so they can be selected, moved, edited, or deleted like standard CAD text.
- Text height: `2.5` drawing units.
- Offset: `1.25` drawing units above and right of the clicked point.
- Text style: the drawing’s standard text style.

## Troubleshooting

| Problem | Check |
| --- | --- |
| The **xfTools** tab is missing | Ensure the DLL and `plugin.toml` are in the same folder, then restart OpenCADStudio. |
| The plugin does not load | Confirm you are using OpenCADStudio `v2026.37`; the plugin depends on its API. |
| Labels are too large or small | The first version uses a fixed height of `2.5`; edit individual labels or request a configurable version. |
| The number does not restart | Start `PNUM` with the desired start value, such as `PNUM 1 1 P-`. |
