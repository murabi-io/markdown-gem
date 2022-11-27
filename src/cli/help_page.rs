use anyhow::Result;
use crossterm::style::{Attribute, Color::*};
use termimad::{minimad::*, *};

use crate::cli::action::Action;
use crate::cli::keybindings::KeyBindings;
use crate::cli::W;

static TEMPLATE: &str = r#"

# gem ${version}

**gem** is a code chunk executor, it runs your markdown files executing your code chunks and outputing results.

See *https://github.com/gem-io/gem* for a complete guide.

|:-:|:-:
|**action**|**shortcuts**
|:-|:-:
${keybindings
|${action}|${keys}
}
|-:

"#;

pub struct HelpPage {
    skin: MadSkin,
    expander: OwningTemplateExpander<'static>,
    template: TextTemplate<'static>,
    scroll: usize,
}

impl HelpPage {
    pub fn new(keybindings: &KeyBindings) -> Self {
        let mut skin = MadSkin::default();
        skin.paragraph.align = Alignment::Center;
        skin.italic = CompoundStyle::new(Some(AnsiValue(204)), None, Attribute::Bold.into());
        skin.table.align = Alignment::Center;
        let mut expander = OwningTemplateExpander::new();
        expander.set("version", env!("CARGO_PKG_VERSION"));
        let mut bindings: Vec<(String, String)> = keybindings
            .build_reverse_map()
            .into_iter()
            .map(|(action, cks)| {
                let action = match action {
                    Action::Internal(internal) => internal.to_string(),
                };
                let cks: Vec<String> = cks.iter().map(|ck| format!("*{ck}*")).collect();
                let cks = cks.join(" or ");
                (action, cks)
            })
            .collect();
        bindings.sort_by(|a, b| a.0.cmp(&b.0));
        for (action, key) in bindings.drain(..) {
            expander
                .sub("keybindings")
                .set_md("keys", key)
                .set_md("action", action);
        }
        let template = TextTemplate::from(TEMPLATE);
        Self {
            skin,
            expander,
            template,
            scroll: 0,
        }
    }

    /// draw the state on area
    pub fn draw(&mut self, w: &mut W, area: &Area) -> Result<()> {
        let text = self.expander.expand(&self.template);
        let fmt_text = FmtText::from_text(&self.skin, text, Some((area.width - 1) as usize));
        let mut text_view = TextView::from(area, &fmt_text);
        self.scroll = text_view.set_scroll(self.scroll);
        Ok(text_view.write_on(w)?)
    }
}
