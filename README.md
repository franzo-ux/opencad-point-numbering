# OpenCADStudio Point Numbering

> **Experimental / work in progress.** This plugin is provided for testing and evaluation. Validate the output on copies of your drawings before using it in production.

An [OpenCADStudio](https://github.com/HakanSeven12/OpenCADStudio) plugin that places incrementing text labels by clicking points in the drawing.

- [Guida rapida in italiano](docs/guide.it.md)
- [Quick guide in English](docs/guide.en.md)

## Install

This plugin is built for OpenCADStudio `v2026.37` and Rust `1.98.1`.

### Windows release

Download `opencad.point_numbering-windows-x86_64.dll` and `plugin.toml` from the [latest release](https://github.com/franzo-ux/opencad-point-numbering/releases/latest), then copy both files to:

```text
%APPDATA%\OpenCADStudio\plugins\opencad.point_numbering\
```

Restart OpenCADStudio.

### Linux source build

```sh
cargo build --release
mkdir -p ~/.config/OpenCADStudio/plugins/opencad.point_numbering
cp target/release/libopencad_point_numbering.so \
  ~/.config/OpenCADStudio/plugins/opencad.point_numbering/
cp plugin.toml ~/.config/OpenCADStudio/plugins/opencad.point_numbering/
```

Restart OpenCADStudio.

## Use

Click **Number points** in the **Numbering** ribbon tab, or run:

```text
PNUM [start] [increment] [prefix]
```

Examples:

```text
PNUM
PNUM 1 1 P-
PNUM 100 10 PT-
```

The command accepts vertices and empty drawing locations. It creates a text label at a fixed upper-right offset, then retains the next number until OpenCADStudio closes. Press Enter or Esc to finish.

## Development

```sh
cargo test
cargo build --release
```

## License

GPL-3.0-only. See [LICENSE](LICENSE).
