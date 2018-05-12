import {Point as P} from 'phaser'

export default class PosVelComponent {
  constructor ({pos, vel, fric, max}) {
    this.pos = pos
    this.vel = vel || new P()
    this.fric = fric || new P(1, 1)
    this.max = max
    this.clamp()
  }

  clamp () {
    const maxVel = this.max
    const vel = this.vel

    if (vel.getMagnitude() > maxVel) {
      vel.setMagnitude(maxVel)
    }
  }

  update () {
    this.clamp()

    let { max, pos, vel, fric } = this

    this.pos = P.add(pos, vel)

    let newv = P.multiply(vel, fric)
    this.vel = newv

    this.clamp()

    if (this.pos.x < 0) {
      this.pos.x = 0
    }

    if (this.pos.y < 0) {
      this.pos.y = 0
    }

    if (max) {
      let {x, y} = max
      if (this.pos.x > x) {
        this.pos.x = x
      }

      if (this.pos.y > y) {
        this.pos.y = y
      }
    }
  }

  getVelMagnitude () {
    return this.vel.getMagnitude()
  }

  addVel (v) {
    P.add(v, this.vel, this.vel)
    this.clamp()
  }

  setVel (v) {
    this.val = v
    this.clamp()
  }

  setFric (v) {
    this.fric = v
  }

  zeroVel () {
    this.vel.x = 0
    this.vel.y = 0
  }

  noFric () {
    this.fric.x = 1
    this.fric.y = 1
  }
}
