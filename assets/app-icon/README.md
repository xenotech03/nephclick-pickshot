# Nephclick Picshot app icon

A **lynx**, frontal head-and-shoulders portrait with an alert gaze, in the style of an engraving: the
Crafting Apps "owl template" (full-bleed colour field, tight portrait, the figure in paper with ink line
work and a crisp ink contour, no roundel or border).

## Palette (exactly three colours)

| Colour | Hex | Used for |
|---|---|---|
| Ink | `#0b0b0c` | line work and the contour |
| Paper | `#efe9dc` | the figure (the lynx) |
| Nephclick Picshot amber | `#f2a516` | the full-bleed field (the app colour) |

## Geometry

`viewBox="0 0 512 512"`, a rounded-square tile with `rx=112`, clipped; the colour fills the whole tile.
macOS files (`.icns`, `picshot-macos-512.png`) add Apple's transparent margin (an 824/1024 body);
Windows and Linux files use the full-bleed tile so the lynx reads at 16-48 px.

## Provenance

The owner's original drawing, made in ArtCraft (2880 px, keyed to the three colours) and
vectorised. `picshot.svg` here is the canonical artwork. Licence:
`LICENSE.txt` (MIT OR Apache-2.0, like the repo).

## Files

| File | What |
|---|---|
| `picshot.svg` | canonical vector, traced at 2048 px (about 770 KB) |
| `picshot-small.svg` | lighter vector, traced at 1024 px (about 360 KB) |
| `picshot-1024.png` | 1024 px render (store listings, docs) |
| `picshot-macos-512.png` | runtime window/Dock icon on macOS (embedded by `apps/picshot/src/main.rs`) |
| `picshot.icns` | macOS bundle icon (`CFBundleIconFile`) |
| `picshot.ico` | Windows icon, 16-256 px, embedded in `picshot.exe` by `apps/picshot/build.rs` |
| `hicolor/<n>x<n>/apps/com.nephclick.picshot.png` | Linux icon theme, 16-512 px; the 256 px one is also the runtime window icon on Windows and Linux |
| `hicolor/scalable/apps/com.nephclick.picshot.svg` | Linux scalable icon (the small SVG) |

The app id is `com.nephclick.picshot`: the Wayland app id, the `.desktop` file
(`packaging/linux/com.nephclick.picshot.desktop`, `Icon=com.nephclick.picshot`) and the hicolor icon
name. To install on Linux, copy `hicolor/` into `/usr/share/icons/hicolor/` (or `~/.local/share/icons/hicolor/`)
and the `.desktop` file into `applications/`.

## Regenerate

Edit `picshot.svg` (or `picshot-small.svg`), then run `packaging/icons.sh` (needs `resvg`; `iconutil`
on macOS for the `.icns`; the `.ico` is packed by `cargo xtask ico`). Every output is committed.
