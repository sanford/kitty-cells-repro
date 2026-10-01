//! ratatui-image, forced to the Kitty protocol, drawing one picture.
//!
//! In Kitty and Ghostty the picture shows. In iTerm2 (tested on 3.7.3, which
//! draws Kitty's unicode-placeholder pictures) the area stays blank, because
//! the transmission has no `c=`/`r=` and iTerm2 doesn't infer the size in
//! cells from the pixel size the way Kitty does.
//!
//! Press any key to quit.

use std::time::Duration;

use crossterm::event::{self, Event};
use kitty_cells_repro::test_image;
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Paragraph},
};
use ratatui_image::{
    Image, Resize,
    picker::{Picker, ProtocolType},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut picker = Picker::from_query_stdio()?;
    let detected = picker.protocol_type();
    picker.set_protocol_type(ProtocolType::Kitty);

    let (cw, ch) = (
        picker.font_size().width as u32,
        picker.font_size().height as u32,
    );
    let (cols, rows) = (40u16, 10u16);
    let img = test_image(cols as u32 * cw, rows as u32 * ch);
    let protocol = picker.new_protocol(
        img,
        ratatui::layout::Size::new(cols, rows),
        Resize::Fit(None),
    )?;

    let mut terminal = ratatui::init();
    let result = (|| -> std::io::Result<()> {
        loop {
            terminal.draw(|f| {
                let [top, pic] =
                    Layout::vertical([Constraint::Length(4), Constraint::Length(rows + 2)])
                        .areas(f.area());
                f.render_widget(
                    Paragraph::new(format!(
                        "detected protocol: {detected:?}, forced: Kitty\n\
                         capabilities: {:?}\n\
                         A {cols}x{rows}-cell gradient should be in the box below. Any key quits.",
                        picker.capabilities()
                    )),
                    top,
                );
                let block = Block::bordered().title("ratatui-image Kitty");
                let inner = block.inner(pic);
                f.render_widget(block, pic);
                f.render_widget(Image::new(&protocol), inner);
            })?;
            if event::poll(Duration::from_millis(250))?
                && let Event::Key(_) = event::read()?
            {
                return Ok(());
            }
        }
    })();
    ratatui::restore();
    Ok(result?)
}
