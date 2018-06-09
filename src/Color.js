function clamp (v) {
  if (v < 0.0) {
    v = 0.0
  }

  if (v > 1.0) {
    v = 1.0
  }

  return v
}

function lerp (a, b, t) {
  return a + ((b - a) * t)
}

export default class Color {
  constructor (r, g, b) {
    this.r = r
    this.g = g
    this.b = b
  }

  blend (o, t, dest) {
    if (!dest) {
      dest = this
    }

    dest.r = lerp(this.r, o.r, t)
    dest.g = lerp(this.g, o.g, t)
    dest.b = lerp(this.b, o.b, t)
  }

  asRGBStr () {
    let r = Math.floor(clamp(this.r) * 255)
    let g = Math.floor(clamp(this.g) * 255)
    let b = Math.floor(clamp(this.b) * 255)

    return `rgb(${r}, ${g}, ${b})`
  }
}
