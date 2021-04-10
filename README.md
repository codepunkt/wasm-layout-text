# @codepunkt/wasm-layout-text

## Key features

This package layouts text and returns the result as an `UInt8Array` of pixels. It supports:

- custom image dimensions
- custom (TrueType) font, font size and color
- text positioning, custom text bounding box
- any combination of vertical and horizontal alignment

## Usage

The package provides a `render` function that returns an `UInt8Array` of pixels:

```js
const { readFileSync } = require("fs");
const { join } = require("path");
const wlt = require("@codepunkt/wasm-layout-text");

const font = readFileSync(join(__dirname, "myfont.ttf"));

const buffer = wlt.render(
  // text, size, color, ttf font buffer
  new wlt.Text("Hello world", 64, new wlt.RgbColor(91, 214, 123), font),
  // image dimension
  new wlt.Dimension(1200, 630),
  // text bounds
  new wlt.Dimension(1100, 530),
  // text position
  new wlt.Position(50, 50),
  // text alignment
  new wlt.Alignment(wlt.HorizontalAlign.Left, wlt.VerticalAlign.Top)
);
```

You can then use [Jimp](https://github.com/oliver-moran/jimp) or other image processing libraries to

- load this buffer
- combine it with other buffers (such as a background image or additional text generated by this package)
- save to image file

## Contributing

Contributions are welcome! A JavaScript usage example with Jimp is available in the `example` directory.

There are a few `make` tasks, most notably:

- `make build`: will build rust code from `./src` to JavaScript code in `./pkg` (using [wasm-pack](https://github.com/rustwasm/wasm-pack))
- `make node`: executes the `example` code with Node.js and uses the local JavaScript package from `.pkg` to render all alignment permutations (expects the `./pkg` folder to exist)
