const fs = require('fs')
const path = require('path')
const Jimp = require('jimp')
const {
  createAlignment,
  createDimension,
  createPosition,
  createRgbColor,
  render,
  HorizontalAlign,
  VerticalAlign,
} = require('../pkg')

const width = 1200
const height = 630
const padding = 50

const examples = [
  {
    name: 'left_top',
    position: createPosition(padding, padding),
    align: createAlignment(HorizontalAlign.Left, VerticalAlign.Top),
  },
  {
    name: 'left_center',
    position: createPosition(padding, height / 2),
    align: createAlignment(HorizontalAlign.Left, VerticalAlign.Center),
  },
  {
    name: 'left_bottom',
    position: createPosition(padding, height - padding),
    align: createAlignment(HorizontalAlign.Left, VerticalAlign.Bottom),
  },
  {
    name: 'center_top',
    position: createPosition(width / 2, padding),
    align: createAlignment(HorizontalAlign.Center, VerticalAlign.Top),
  },
  {
    name: 'center_center',
    position: createPosition(width / 2, height / 2),
    align: createAlignment(HorizontalAlign.Center, VerticalAlign.Center),
  },
  {
    name: 'center_bottom',
    position: createPosition(width / 2, height - padding),
    align: createAlignment(HorizontalAlign.Center, VerticalAlign.Bottom),
  },
  {
    name: 'right_top',
    position: createPosition(width - padding, padding),
    align: createAlignment(HorizontalAlign.Right, VerticalAlign.Top),
  },
  {
    name: 'right_center',
    position: createPosition(width - padding, height / 2),
    align: createAlignment(HorizontalAlign.Right, VerticalAlign.Center),
  },
  {
    name: 'right_bottom',
    position: createPosition(width - padding, height - padding),
    align: createAlignment(HorizontalAlign.Right, VerticalAlign.Bottom),
  },
]

const color = createRgbColor(90, 212, 112)
const size = createDimension(width, height)
const bounds = createDimension(width - padding * 2, height - padding * 2)

;(async () => {
  for (let { name, position, align } of examples) {
    const buffer = render(
      'Musings about web development and cloud technology.',
      100,
      color,
      fs.readFileSync(path.join(__dirname, 'OpenSans-Regular.ttf')),
      size,
      bounds,
      position,
      align
    )

    const image = new Jimp({ data: buffer, width, height })

    await image
      .quality(100)
      .writeAsync(path.join(__dirname, `results/${name}.png`))
  }
})()
