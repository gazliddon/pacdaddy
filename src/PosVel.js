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
    this.pos = P.add(this.pos, this.vel)
    P.add(this.pos, this.vel, this.pos)
    P.multiply(this.vel, this.fric, this.vel)
    this.clamp()
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
