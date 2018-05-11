import {Point as P} from 'phaser'

function keysToVel (keys) {
  let p = new P(0, 0)

  if (keys.up.isDown) {
    p.y = -1
  } else if (keys.down.isDown) {
    p.y = 1
  }

  if (keys.left.isDown) {
    p.x = -1
  } else if (keys.right.isDown) {
    p.x = 1
  }

  p.normalize()
  return p
}

export default class StickComponnet {
  constructor ({keys}) {
    this.keys = keys
  }

  update () {
    this.vel = keysToVel(this.keys)
  }
}
