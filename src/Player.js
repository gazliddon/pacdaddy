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

    this.ev('start')
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
    this.comps.posVel.setFric(new P(0.7, 0.7))
  }

  onEatFruit (_, first) {
    let scale = this.comps.scale.target
    P.multiply(scale, new P(1.01, 1.01), scale)
  }

  update () {
    let {posVel, scale, spr, stick, text} = this.comps

    stick.update()

    if (stick.vel.getMagnitude() < 0.0000001) {
      this.ev('stickidle')
    } else {
      this.ev('stickmove', stick.vel)
    }

    posVel.update()

    if (posVel.getVelMagnitude() < 0.1) {
      this.ev('stopped')
    }

    scale.setFlipX(posVel.vel.x < 0)
    scale.update()

    spr.setScale(scale.getScale())
    spr.setPos(posVel.pos)
    spr.update()

    text.setPos(new P(posVel.pos.x, posVel.pos.y - 10))
    text.update()
  }
}
