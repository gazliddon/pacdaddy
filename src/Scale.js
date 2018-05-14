import {Point as P} from 'phaser'

export default class ScaleComponent {
  constructor (scale) {
    this.flipX = false
    this.flipY = false
    this.target = scale
    this.scale = scale
    this.div = 20
  }

  update () {
    // bad lerp
    let diff = (this.target - this.scale) / this.div
    this.scale = this.scale + diff
  }

  setTarget (n) {
    this.target = n
  }

  setScale (n) {
    this.target = n
    this.scale = n
  }

  setFlipX (val) {
    this.flipX = val
  }

  setFlipY (val) {
    this.flipY = val
  }

  getScale () {
    let scaleX = this.scale * (this.flipX ? -1 : 1)
    let scaleY = this.scale * (this.flipY ? -1 : 1)
    return new P(scaleX, scaleY)
  }
}
