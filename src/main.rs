use cli_log::*;
use crossterm::{execute, style::Color, terminal};
use minimad::Line;
use minimad::{clean, Text};
use termimad::*;

use {
    anyhow::{self},
    crokey::key,
    crossterm::{
        cursor,
        event::{DisableMouseCapture, EnableMouseCapture},
        terminal::{EnterAlternateScreen, LeaveAlternateScreen},
        QueueableCommand,
    },
    std::io::{stdout, BufWriter, Write},
    termimad::*,
};

mod app;
mod cli;
mod executor;
mod fenced_attributes;
mod state;
mod view;

#[macro_use]
extern crate cli_log;

fn main() -> anyhow::Result<()> {
    init_cli_log!();
    cli::cli::run()?;
    info!("bye");
    Ok(())
}

// /// run the event loop, in a terminal which must be in alternate
// fn run_in_alternate<W: Write>(w: &mut W) -> anyhow::Result<()> {
//     let mut view = view::View::new(Area::full_screen());
//     view.queue_on(w)?;
//     w.flush()?;
//     let md = clean::lines(MD);
//     let text = TextWithCodeChunks::from_md_lines(md.into_iter());
//     for line in text.lines {
//         let (l, code) = line;
//     }
//     let event_source = EventSource::new()?;
//     for timed_event in event_source.receiver() {
//         let mut quit = false;
//         if timed_event.is_key(key!(ctrl - q)) {
//             quit = true;
//         } else if view.apply_timed_event(timed_event) {
//             view.queue_on(w)?;
//             w.flush()?;
//         }
//         event_source.unblock(quit); // Don't forget to unblock the event source
//         if quit {
//             break;
//         }
//     }
//     Ok(())
// }

static MD: &str = r#"
----

# Markdown Rendering on Terminal

Here's the code to print this markdown block in the terminal:

```
let mut skin = MadSkin::default();
skin.set_headers_fg(rgb(255, 187, 0));
skin.bold.set_fg(Yellow);
skin.italic.set_fgbg(Magenta, rgb(30, 30, 40));
skin.bullet = StyledChar::from_fg_char(Yellow, '⟡');
skin.quote_mark = StyledChar::from_fg_char(Yellow, '▐');
skin.bullet = StyledChar::from_fg_char(Yellow, '⟡');
skin.quote_mark.set_fg(Yellow);
println!("{}", skin.term_text(my_markdown));
```

**Termimad** is built over **Crossterm** and **Minimad**.

----

## Why use Termimad

* *display* static or dynamic *rich* texts
* *separate* your text building code or resources from its styling
* *configure* your colors

## Real use cases


```js {cmd=node output=txt modify_source}
var items = [5,3,7,6,2,9];
function swap(items, leftIndex, rightIndex){
  var temp = items[leftIndex];
  items[leftIndex] = items[rightIndex];
  items[rightIndex] = temp;
}
function partition(items, left, right) {
  var pivot   = items[Math.floor((right + left) / 2)], //middle element
      i       = left, //left pointer
      j       = right; //right pointer
  while (i <= j) {
    while (items[i] < pivot) {
      i++;
    }
    while (items[j] > pivot) {
      j--;
    }
    if (i <= j) {
      swap(items, i, j); //sawpping two elements
      i++;
      j--;
    }
  }
  return i;
}

function quickSort(items, left, right) {
  var index;
  if (items.length > 1) {
    index = partition(items, left, right); //index returned from partition
    if (left < index - 1) { //more elements on the left side of the pivot
      quickSort(items, left, index - 1);
    }
    if (index < right) { //more elements on the right side of the pivot
      quickSort(items, index, right);
    }
  }
  return items;
}
// first call to quick sort
var sortedArray = quickSort(items, 0, items.length - 1);
console.log(sortedArray); //prints [2,3,5,6,7,9]
```

* the help screen of a terminal application
* small snippets of rich text in a bigger application
* terminal app output

## What people say about Termimad

> I find it convenient *[Termimad's author]*

----
"#;
