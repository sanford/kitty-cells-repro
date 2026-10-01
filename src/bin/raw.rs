//! The same Kitty picture, sent three ways with plain escapes, no ratatui.
//!
//!   A: f=32, no c/r    (what ratatui-image 11.1 sends)
//!   B: f=32, c=, r=    (A plus the size in cells)
//!   C: f=100, c=, r=   (PNG, with the size in cells)
//!
//! All three show in Kitty and Ghostty. In iTerm2 3.7.3, A stays blank while
//! B and C show.

use std::io::Write;

use base64::Engine;
use kitty_cells_repro::test_image;

const PLACEHOLDER: char = '\u{10EEEE}';
// The first of Kitty's row/column diacritics, enough for this picture.
const DIACRITICS: [char; 24] = [
    '\u{305}', '\u{30D}', '\u{30E}', '\u{310}', '\u{312}', '\u{33D}', '\u{33E}', '\u{33F}',
    '\u{346}', '\u{34A}', '\u{34B}', '\u{34C}', '\u{350}', '\u{351}', '\u{352}', '\u{357}',
    '\u{35B}', '\u{363}', '\u{364}', '\u{365}', '\u{366}', '\u{367}', '\u{368}', '\u{369}',
];
const COLS: u32 = 24;
const ROWS: u32 = 6;

fn main() {
    // Cell size in pixels, from the terminal; a guess if it won't say.
    let (cw, ch) = match crossterm::terminal::window_size() {
        Ok(s) if s.width > 0 && s.columns > 0 => (
            s.width as u32 / s.columns as u32,
            s.height as u32 / s.rows as u32,
        ),
        _ => (10, 20),
    };
    let img = test_image(COLS * cw, ROWS * ch);
    let rgba = base64::engine::general_purpose::STANDARD.encode(img.to_rgba8().as_raw());
    let mut png = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
        .unwrap();
    let png = base64::engine::general_purpose::STANDARD.encode(&png);
    let (w, h) = (img.width(), img.height());

    let mut out = std::io::stdout().lock();
    writeln!(
        out,
        "cell {cw}x{ch} px, picture {w}x{h} px = {COLS}x{ROWS} cells\n"
    )
    .unwrap();
    let cases = [
        (
            101,
            "A: f=32, no c/r (ratatui-image 11.1)",
            format!("f=32,s={w},v={h}"),
            &rgba,
        ),
        (
            102,
            "B: f=32, c/r given",
            format!("f=32,s={w},v={h},c={COLS},r={ROWS}"),
            &rgba,
        ),
        (
            103,
            "C: f=100 (PNG), c/r given",
            format!("f=100,c={COLS},r={ROWS}"),
            &png,
        ),
    ];
    for (id, label, params, data) in cases {
        transmit(&mut out, id, &params, data);
        writeln!(out, "{label}").unwrap();
        place(&mut out, id);
        writeln!(out).unwrap();
    }
    out.flush().unwrap();
}

/// Sends the picture as `id` with a virtual placement (U=1), in chunks.
fn transmit(out: &mut impl Write, id: u32, params: &str, data: &str) {
    let chunks: Vec<&[u8]> = data.as_bytes().chunks(4096).collect();
    for (i, chunk) in chunks.iter().enumerate() {
        let more = u8::from(i + 1 < chunks.len());
        // q=2 on every chunk, as ratatui-image does: iTerm2 answers the last
        // chunk otherwise.
        write!(out, "\x1b_Gq=2,").unwrap();
        if i == 0 {
            write!(out, "a=T,U=1,i={id},{params},").unwrap();
        }
        write!(
            out,
            "m={more};{}\x1b\\",
            std::str::from_utf8(chunk).unwrap()
        )
        .unwrap();
    }
}

/// Writes the placeholder cells, coloured with the id, each marked with
/// its row and column.
fn place(out: &mut impl Write, id: u32) {
    let [_, r, g, b] = id.to_be_bytes();
    for row in &DIACRITICS[..ROWS as usize] {
        write!(out, "\x1b[38;2;{r};{g};{b}m").unwrap();
        for col in &DIACRITICS[..COLS as usize] {
            write!(out, "{PLACEHOLDER}{row}{col}").unwrap();
        }
        writeln!(out, "\x1b[39m").unwrap();
    }
}
