# Markdown Rendering and Executing on Terminal

Here is a bash command to list the current dir on linux or macos
```shell {cmd=sh, sys=[linux, macos]}
ls -la
sleep 2
```
or on windows:

```shell {sys=[windows]}
Get-ChildItem -Recursive | Sort LastWriteTime 
```

**Murabi** is built using **Termimad** which in turn is built over **Crossterm** and **Minimad**.

----

## Why use Murabi

* *automate* setup markdown files
* *maintain* your documentation and make sure the examples are working
* *improve* your user experience

## Real use cases

It can very, from example showcase, e.g. quick sort implementation in JS

```js {cmd=node}

const timer = ms => new Promise( res => setTimeout(res, ms));

// wait for 1 second
timer(1000).then(_=> console.log("done"));

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

// wait for 1 second
timer(1000).then(_=> console.log(sortedArray)); //prints [2,3,5,6,7,9]

```

to some system specific setup instructions.

linux:

```shell {cmd=sh, sys=[linux], with_sudo}
uname
nproc
uptime
```

macos:
```shell {sys=[macos]}
uname
sysctl -n hw.ncpu
last reboot
```

windows:
```shell {sys=[windows]}
uname
echo %NUMBER_OF_PROCESSORS%
systeminfo | find "System Boot Time:"
```

or maybe output of the benchma**r**k results for the commands:


```shell {cmd=sh, sys=[linux, macos]}
time ping google.com -c 10
```
and of course you can display tables:
| Tables   |      Are      |  Cool |
|----------|:-------------:|------:|
| col 1 is |  left-aligned | $1600 |
| col 2 is |    centered   |   $12 |
| col 3 is | right-aligned |    $1 |


## What else can you do?

> Let your imagination go wild
