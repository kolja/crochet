# crochet helper

Let's say you want to crochet something round, like a coaster or a hat.
With every row, you will have to increase the number of stitches by 2𝛑.

So, sometimes you'll have to fork out from a stitch to double it: six times on some rows, seven times on others.
But which row gets which number of stitches? And how many "regular" stitches go in between?

This crochet helper will tell you.
Just say if you want to crochet a *circle* or a *sphere* (in the latter case, you will also have to specify the radius), and it will print a pattern of stitches.

## Usage

```
Usage: crochet [OPTIONS] <TYPE> [ROW]

Arguments:
  <TYPE>  crochet type ('circle' or 'sphere')
  [ROW]   starting row number [default: 0]

Options:
  -s, --sphere-radius <RADIUS>  sphere radius (for 'sphere' type)
  -h, --help                    Print help
  -V, --version                 Print version``
```

## Example
This is what the output looks like for a circle:
First is the row number, then the number of stitches you *have* in that row and the number of stitches you *want* in the next row.
Then the pattern of stitches is shown, where `V` means a stitch that is doubled and `|` is a regular stitch.
```
 > circle
 0:   0   6 -> VVV
 1:   6  13 -> VVVVVV
 2:  13  19 -> V||V|V|V|V|V|
 3:  19  25 -> V|||V||V||V||V||V||
 4:  25  31 -> V||||V|||V|||V|||V|||V|||
 5:  31  38 -> V||||V|||V||||V|||V||||V|||V|||
 6:  38  44 -> V||||||V|||||V|||||V||||||V|||||V|||||
 7:  44  50 -> V|||||||V||||||V||||||V|||||||V||||||V||||||
 8:  50  57 -> V|||||||V||||||V||||||V||||||V||||||V||||||V||||||
```

## License

MIT
