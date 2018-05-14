import {Point as P} from 'phaser'

import Obj from './Obj'
import PosVelComponent from './PosVel'
import ScaleComponent from './Scale'
import StickComponnet from './Stick'
import SprComponent from './Spr'
import TextSprComponent from './TextSpr'

const table = {
  start: { nothing: 'idle' },
  stickmove: { idle: 'move', slide: 'move', move: 'move' },
  stickidle: { move: 'slide' },
  stopped: { slide: 'idle', move: 'idle' },

  hitFruit: {
    idle: 'eatFruit',
    slide: 'eatFruit',
    move: 'eatFruit'
  }
}

export default class Player extends Obj {
  constructor ({game, id, pos, group, name}) {
    super({game, type: 'player', id, table})

    let keys = game.input.keyboard.createCursorKeys()

    this.comps = {
      scale: new ScaleComponent(1),
      posVel: new PosVelComponent({pos, max: 2}),
      stick: new StickComponnet({keys}),
      spr: new SprComponent({
        game, pos, group, asset: 'ms'
      }),
      text: new TextSprComponent({game, pos, text: name})
    }
    this.name = name
    this.score = 0
    this.scale = 1

    this.ev('start')
  }

  setScale (n) {
    let {scale} = this.comps
    scale.setTarget(n)
    this.scale = n
  }

  getScale () {
    return this.scale
  }

  getSprite () {
    return this.comps.spr.spr
  }

  onIdle () {
    let {posVel, spr} = this.comps
    posVel.zeroVel()
    spr.playAnim('stopped')
  }

  onMove (vel) {
    let {posVel, spr} = this.comps
    P.multiply(vel, new P(0.8, 0.8), vel)
    posVel.addVel(vel)
    posVel.noFric()
    spr.playAnim('wacca')
  }

  onSlide () {
    this.comps.posVel.setFric(new P(0.9, 0.9))
  }

  onEatFruit (_, first) {
    let scale = this.comps.scale.target
    P.multiply(scale, new P(1.01, 1.01), scale)
  }

  update () {
    let {game} = this
    let {posVel, scale, spr, stick, text} = this.comps

    let sx = game.camera.scale.x
    let sy = game.camera.scale.y
    let x = game.input.activePointer.worldX / sx
    let y = game.input.activePointer.worldY / sy

    let mp = new P(x, y)

    stick.update()

    // let vel = stick.vel

    let vel = P.subtract(mp, posVel.pos)

    if (vel.getMagnitude() < 20) {
      this.ev('stickidle')
    } else {
      this.ev('stickmove', vel)
    }

    posVel.update()

    if (posVel.getVelMagnitude() < 0.001) {
      this.ev('stopped')
    }

    scale.setFlipX(posVel.vel.x < 0)
    scale.update()

    spr.setScale(scale.getScale())
    spr.setPos(posVel.pos)
    spr.update()

    text.setPos(new P(posVel.pos.x, posVel.pos.y - 10 * scale.scale))
    text.update()
  }
}
