# kitty-cells-repro

ratatui-image's Kitty pictures are blank in iTerm2, because the transmission
leaves out the placement's size in cells (`c=`/`r=`).

ratatui-image 11.1.0 sends a virtual placement for unicode placeholders as

    a=T,U=1,f=32,t=d,s={w},v={h}

Kitty and Ghostty infer the size in cells from the pixel size. iTerm2 (tested on
3.7.3) doesn't, and draws nothing. iTerm2 answers the Kitty graphics query, so
ratatui-image's `Picker` picks `ProtocolType::Kitty` there on its own. Any
ratatui-image app in iTerm2 shows blank pictures by default.

## Run

    cargo run              # ratatui-image, Kitty protocol: blank box in iTerm2
    cargo run --bin raw    # the same picture sent three ways with plain escapes

`raw` sends:

| case | parameters               | Kitty | Ghostty                | iTerm2 3.7.3 |
|------|--------------------------|-------|------------------------|--------------|
| A    | `f=32`, no `c`/`r` (ratatui-image today) | ✅ | ✅, but sized by pixels* | ❌ blank |
| B    | `f=32`, `c=`/`r=` given  | ✅    | ✅                     | ✅           |
| C    | `f=100` (PNG), `c=`/`r=` | ✅    | ✅                     | ✅           |

\* When the picture's pixel size doesn't match the terminal's cell size exactly,
A comes out at the wrong size, because the terminal works the cells out from the
pixels. Here the cell size comes from `TIOCGWINSZ`, which on a Retina display in
Ghostty gives points, so A is drawn at half size. Giving `c`/`r` pins it to
the cells the placeholders cover.

## With the fix

Two patches, the same change: pass the `Size` that `Kitty::new` /
`StatefulKitty::resize_encode` already have through to the transmit, and add
`c={cols},r={rows}`.

- `ratatui-image-main-c-r.patch`: against `main` (v12.0.0-rc.0), covering both the
  base64 (`t=d`) and shared-memory (`t=s`) paths
- `ratatui-image-11.1.0-c-r.patch`: against v11.1.0, which this crate depends on

To try it:

    git clone https://github.com/ratatui/ratatui-image && cd ratatui-image
    git checkout v11.1.0 && git apply ../kitty-cells-repro/ratatui-image-11.1.0-c-r.patch
    cd ../kitty-cells-repro
    cargo run --config 'patch.crates-io.ratatui-image.path="../ratatui-image"'

With either patch, the picture shows in iTerm2 3.7.3 and still shows in Kitty
and Ghostty.

## Screenshots

- iTerm2, `raw`: A blank, B and C drawn: `screenshots/iterm2-raw-A-blank-B-C-drawn.png`
- iTerm2, ratatui-image 11.1.0: `screenshots/iterm2-ratatui-image-11.1-blank.png`
- iTerm2, ratatui-image main with the patch: `screenshots/iterm2-ratatui-image-main-patched.png`
- Ghostty (Retina), `raw`: A at half size: `screenshots/ghostty-raw-A-half-size.png`
