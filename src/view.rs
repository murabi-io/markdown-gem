use crossterm::style::Attribute;
use crossterm::style::Color::{AnsiValue, Magenta, Yellow};

use anyhow;
use termimad::*;

use crate::cli::cli::W;
use crate::cli::help_line::HelpLine;
use crate::cli::help_page::HelpPage;
use crate::cli::keybindings::KeyBindings;
use crate::executor::execution_plan::ExecutionItem;
use crate::minimad::{clean, Composite, Text};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ViewLine {
    LineOutput(String),
    CodeOutput(String),
    ExecutionLine,
    // We will need to integrate it to the execution output
    #[allow(dead_code)]
    ClearLine,
}

impl From<ExecutionItem> for ViewLine {
    fn from(item: ExecutionItem) -> Self {
        match item {
            ExecutionItem::OutputCode(code) => ViewLine::CodeOutput(code),
            ExecutionItem::OutputString(str) => ViewLine::LineOutput(str),
            ExecutionItem::Execute(e) => ViewLine::LineOutput(e.code),
        }
    }
}

/// The view covering the whole terminal, with its widgets and current state
pub struct View {
    width: usize,
    height: usize,
    render_skin: MadSkin,
    status_skin: MadSkin,
    render_area: Area, // where the markdown will be rendered
    // status_area: Area, // where the markdown will be rendered
    /// the tool building the help line
    help_line: HelpLine,
    /// the help page displayed over the rest, if any
    help_page: HelpPage,
    show_help_page: bool,
    executing: bool,
    lines: Vec<ViewLine>,
    /// number of lines hidden on top due to scroll
    scroll: usize,
}

impl View {
    pub fn new(keybindings: &KeyBindings) -> Self {
        let mut render_skin = MadSkin::default();
        render_skin.set_headers_fg(AnsiValue(178));
        render_skin.bold.set_fg(Yellow);
        render_skin.italic.set_fg(Magenta);
        render_skin.scrollbar.thumb.set_fg(AnsiValue(178));
        render_skin.code_block.align = Alignment::Center;
        render_skin.table.align = Alignment::Center;
        render_skin.scrollbar.thumb.set_fg(AnsiValue(178));

        let (width, height) = terminal_size();

        let render_area = Area::new(0, 0, width, if height > 1 { height - 1 } else { 0 });
        // let status_area = Area::new(0, height, width, 1);
        // render_area.pad_for_max_width(120); // we don't want a too wide text column

        let mut status_skin = MadSkin::default();
        status_skin
            .paragraph
            .set_fgbg(AnsiValue(252), AnsiValue(239));
        status_skin.italic = CompoundStyle::new(Some(AnsiValue(204)), None, Attribute::Bold.into());

        let help_line = HelpLine::new(keybindings);
        let help_page = HelpPage::new(keybindings);

        Self {
            width: width as usize,
            height: height as usize,
            render_skin,
            status_skin,
            render_area,
            // status_area,
            help_line,
            help_page,
            show_help_page: false,
            executing: false,
            lines: vec![],
            scroll: 0,
        }
    }

    /// close the help and return true if it was open,
    /// return false otherwise
    pub fn close_help(&mut self) -> bool {
        self.show_help_page = false;
        true
    }
    pub fn is_help(&self) -> bool {
        self.show_help_page
    }

    pub fn toggle_help(&mut self) {
        self.show_help_page = !self.show_help_page;
    }

    /// draw the grey line containing the keybindings indications
    pub fn draw_help_line(&mut self, w: &mut W) -> anyhow::Result<()> {
        let markdown = self.help_line.markdown(self);
        if self.height > 1 {
            self.status_skin.write_composite_fill(
                w,
                Composite::from_inline(&markdown),
                self.width.into(),
                Alignment::Left,
            )?;
        }
        Ok(())
    }

    pub fn execution_starts(&mut self) {
        self.executing = true;
    }
    pub fn execution_stops(&mut self) {
        self.executing = false;
    }

    /// draw "executing...", the error code if any, or a blank line
    pub fn draw_executing(&mut self) -> () {
        self.lines.push(ViewLine::ExecutionLine);
    }

    pub fn write_command_output(&mut self, w: &mut W, output: String) -> anyhow::Result<()> {
        let line = ViewLine::CodeOutput(output);
        self.lines.push(line);

        self.scroll_to_bottom();
        self.draw(w, Some(self.scroll as i32))
    }

    pub fn write_on(&mut self, w: &mut W, output: ExecutionItem) -> anyhow::Result<()> {
        let line: ViewLine = output.into();
        self.lines.push(line);

        self.scroll_to_bottom();
        self.draw(w, Some(self.scroll as i32))
    }

    pub fn draw(&mut self, w: &mut W, lines_count: Option<i32>) -> anyhow::Result<()> {
        if self.show_help_page {
            self.help_page.draw(w, &self.render_area)?;
        } else {
            let (md_lines, _) = self.lines.iter().fold((vec![], None), Self::build_md);
            let text = Text::from_md_lines(md_lines.into_iter());
            let width = if self.render_area.width > 0 {
                self.render_area.width - 1
            } else {
                0
            };
            let fmt_text = FmtText::from_text(&self.render_skin, text, Some(width as usize));

            let mut text_view = TextView::from(&self.render_area, &fmt_text);
            if lines_count.is_some() {
                text_view.scroll = self.scroll;
                text_view.try_scroll_lines(lines_count.unwrap());
                self.scroll = text_view.scroll;
            } else {
                text_view.try_scroll_lines(self.scroll as i32);
            }

            // ensure that there is space for output
            if self.width > 0 && self.height > 0 {
                text_view.write_on(w)?;
            }
        }
        Ok(())
    }

    fn build_md<'a>(
        carry: (Vec<&'a str>, Option<&'a ViewLine>),
        current: &'a ViewLine,
    ) -> (Vec<&'a str>, Option<&'a ViewLine>) {
        let (mut acc, prev) = carry;
        match (current, prev) {
            (ViewLine::CodeOutput(l), None)
            | (ViewLine::LineOutput(l), Some(ViewLine::CodeOutput(_)))
            | (ViewLine::CodeOutput(l), Some(ViewLine::LineOutput(_))) => {
                acc.push("```");
                acc.push(l.as_str());
                (acc, Some(current))
            }
            (v @ ViewLine::ExecutionLine, _) => {
                let mut executin_line = clean::lines(
                    r#"
                    ~~~~~~~~~
                    executing...
                    ~~~~~~~~~
                    "#,
                );
                acc.append(&mut executin_line);
                (acc, Some(v))
            }
            (v @ ViewLine::ClearLine, _) => {
                acc.push("```");
                (acc, Some(v))
            }
            (ViewLine::LineOutput(l), _) | (ViewLine::CodeOutput(l), _) => {
                acc.push(l.as_str());
                (acc, Some(current))
            }
        }
    }

    /// set the scroll amount.
    /// lines_count can be negative
    pub fn try_scroll_lines(&mut self, w: &mut W, lines_count: i32) -> anyhow::Result<()> {
        self.draw(w, Some(lines_count))
    }

    /// set the scroll amount.
    /// lines_count can be negative
    pub fn try_scroll_pages(&mut self, w: &mut W, pages_count: i32) -> anyhow::Result<()> {
        self.draw(w, Some(pages_count * i32::from(self.render_area.height)))
    }

    fn scroll_to_bottom(&mut self) {
        let ch = self.content_height();
        let ph = self.page_height();
        self.scroll = if ch > ph { ch - ph - 1 } else { 0 };
    }
    fn fix_scroll(&mut self) {
        let scroll = self.scroll;
        let content_height = self.content_height();
        let page_height = self.page_height();
        self.scroll = if content_height > page_height {
            scroll.min(content_height - page_height - 1)
        } else {
            0
        };
    }
    /// get the scroll value needed to go to the last item (if any)
    fn get_last_item_scroll(&self) -> usize {
        self.lines.len()
    }
    fn try_scroll_to_last_top_item(&mut self) {
        self.scroll = self.get_last_item_scroll();
        self.fix_scroll();
    }
    fn content_height(&self) -> usize {
        self.lines.len()
    }
    fn page_height(&self) -> usize {
        self.height.max(3) as usize - 3
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.try_scroll_to_last_top_item();
    }
}
