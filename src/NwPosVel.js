import {Point as P} from 'phaser'

export default class NwPosVelComponent {
  constructor ({pos, vel, time}) {
    this.startPos = pos
    this.pos = pos
    this.vel = vel
    this.startTime = time
  }

  update (now) {
    let {startPos, vel, time} = this

    let diff = now - time

    let diff60 = diff / (1000000000 / 60)

    let deltaPos = P.multiply(vel, diff60)

    this.pos = P.add(deltaPos, startPos)

    return this.pos.clone()
  }
}
