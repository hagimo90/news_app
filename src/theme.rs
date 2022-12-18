use {
    termimad::crossterm::{
        style::Color::*,
    },
    termimad::*,
};
pub fn default() -> MadSkin {
    // using the struct api
    let mut skin = MadSkin::default();
    skin.bold.set_fg(Yellow);
    skin.italic.set_bg(Rgb{
        r:  28,
        g: 28,
        b: 28,
    });
    skin.set_headers_fg(AnsiValue(178));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.code_block.align = Alignment::Center;
    skin
    }