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

Here is a bash command to list the current dir on linux or macos
```shell {sys=[linux, macos]}
ls -l
```
or on windows:

```shell {sys=[windows]}
Get-ChildItem -Recursive | Sort LastWriteTime 
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
