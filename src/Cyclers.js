import Color from './Color'

function getSampleIndicies (tabSize, t) {
  let i0 = Math.floor(t) % tabSize
  let i1 = (i0 + 1) % tabSize
  let frac = t - Math.floor(t)
  return [i0, i1, frac]
}

let blue = {
  table: [
    new Color(0.5, 0, 0),
    new Color(0.5, 0, 0),
    new Color(1, 0, 0),
    new Color(1, 0.5, 0.5),
    new Color(1, 0, 0)
  ],
  speed: 5.0
}

export default class Cyclers {
  constructor () {
    this.cyclers = {}
    this.add('blue', blue)
  }

  add (name, {table, speed}) {
    this.cyclers[name] = {table, speed}
  }

  get (name, t, phase) {
    let res = new Color(1, 0, 1)
    let cycler = this.cyclers[name]

    if (cycler) {
      let {table, speed} = cycler

      if (!phase) {
        phase = 0.0
      }

      let realT = t * speed + phase

      let [i0, i1, frac] = getSampleIndicies(table.length, realT)
      let c0 = table[i0]
      let c1 = table[i1]
      c0.blend(c1, frac, res)
    }

    return res
  }
}
