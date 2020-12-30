const { readFileSync } = require('fs')
const { join } = require('path')
const Jimp = require('jimp')
const {
  render,
  Alignment,
  HorizontalAlign,
  VerticalAlign,
  Dimension,
  RgbColor,
  Position,
  Text,
} = require('../pkg')

const width = 1200
const height = 630
const padding = 50

const examples = [
  {
    name: 'left_top',
    position: new Position(padding, padding),
    align: new Alignment(HorizontalAlign.Left, VerticalAlign.Top),
  },
  {
    name: 'left_center',
    position: new Position(padding, height / 2),
    align: new Alignment(HorizontalAlign.Left, VerticalAlign.Center),
  },
  {
    name: 'left_bottom',
    position: new Position(padding, height - padding),
    align: new Alignment(HorizontalAlign.Left, VerticalAlign.Bottom),
  },
  {
    name: 'center_top',
    position: new Position(width / 2, padding),
    align: new Alignment(HorizontalAlign.Center, VerticalAlign.Top),
  },
  {
    name: 'center_center',
    position: new Position(width / 2, height / 2),
    align: new Alignment(HorizontalAlign.Center, VerticalAlign.Center),
  },
  {
    name: 'center_bottom',
    position: new Position(width / 2, height - padding),
    align: new Alignment(HorizontalAlign.Center, VerticalAlign.Bottom),
  },
  {
    name: 'right_top',
    position: new Position(width - padding, padding),
    align: new Alignment(HorizontalAlign.Right, VerticalAlign.Top),
  },
  {
    name: 'right_center',
    position: new Position(width - padding, height / 2),
    align: new Alignment(HorizontalAlign.Right, VerticalAlign.Center),
  },
  {
    name: 'right_bottom',
    position: new Position(width - padding, height - padding),
    align: new Alignment(HorizontalAlign.Right, VerticalAlign.Bottom),
  },
]

const size = new Dimension(width, height)
const bounds = new Dimension(width - padding * 2, height - padding * 2)
const text = new Text(
  'Musings about web development and cloud technology.',
  100,
  new RgbColor(90, 212, 112),
  readFileSync(join(__dirname, 'OpenSans-Regular.ttf'))
)

;(async () => {
  for (let { name, position, align } of examples) {
    await new Jimp({
      data: render(text, size, bounds, position, align),
      width,
      height,
    })
      .quality(100)
      .writeAsync(join(__dirname, `results/${name}.png`))
  }
})()
