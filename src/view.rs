use crate::cli::cli::W;
use crate::Line::Normal;
use crossterm::style::Color::{AnsiValue, Magenta, Yellow};
use termimad::minimad::Line;
use {
    anyhow::{self},
    crokey::key,
    crossterm::{
        event::{Event, KeyEvent, MouseEvent},
        queue,
        terminal::{Clear, ClearType},
    },
    std::io::Write,
    termimad::*,
};

/// The view covering the whole terminal, with its widgets and current state
pub struct View {
    width: usize,
    height: usize,
    render_skin: MadSkin,
}

impl Default for View {
    /// Create the view with all its widgets
    fn default() -> Self {
        let mut render_skin = MadSkin::default();
        render_skin.table.align = Alignment::Center;
        render_skin.set_headers_fg(AnsiValue(178));
        render_skin.bold.set_fg(Yellow);
        render_skin.italic.set_fg(Magenta);
        render_skin.scrollbar.thumb.set_fg(AnsiValue(178));
        render_skin.code_block.align = Alignment::Center;
        let (width, height) = terminal_size();
        Self {
            width: width as usize,
            height: height as usize,
            render_skin,
        }
    }
}

impl View {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write_on<W: Write>(&mut self, w: &mut W, line: Line<'_>) -> anyhow::Result<()> {
        Ok(match line {
            Normal(l) => {
                let mut fc = FmtComposite::from(l, &self.render_skin);
                fc.fill_width(self.width, Alignment::Left, &self.render_skin);
                write!(
                    w,
                    "{}",
                    FmtInline {
                        skin: &self.render_skin,
                        composite: fc,
                    }
                )?;
            }
            _ => (),
        })
    }
}
