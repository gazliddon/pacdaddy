import {Point as P, Sprite as Spr} from 'phaser'

export default class extends Spr {
  constructor ({ game, pos, vel, asset, fric, max }) {
    super(game, pos.x, pos.y, asset)
    this.anchor.setTo(0.5)
    this.pos = pos
    this.vel = vel
    this.fric = fric
    this.max = max
  }

  update () {
    let {max, pos, vel, fric} = this

    pos = P.add(pos, vel)
    P.add(this.pos, vel, pos)
    P.multiply(vel, fric, vel)

    if (pos.x < 0) {
      pos.x = 0
    }

    if (pos.y < 0) {
      pos.y = 0
    }

    if (max) {
      let {x, y} = max
      if (pos.x > x) {
        pos.x = x
      }

      if (pos.y > y) {
        pos.y = y
      }
    }

    this.x = pos.x
    this.y = pos.y
  }
}
