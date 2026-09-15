# Point Numbering

The **Number points** ribbon tool uses the theme-aware `1,2…` glyph and starts an interactive placement command.

- Click a vertex or empty location to add the next label.
- Labels use the current text style at a fixed upper-right offset (height `2.5`, offset `1.25` drawing units).
- Press Enter or Esc to stop. The next value remains available until OpenCADStudio is closed.

## Configure or restart

Use the command line before placement:

```text
PNUM:start,increment,prefix
```

Examples:

```text
PNUM
PNUM:1,1,P-
PNUM:100,10,PT-
```

`PNUM` without arguments continues from the current session value.
