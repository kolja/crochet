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

## License

This project is licensed under the MIT License
