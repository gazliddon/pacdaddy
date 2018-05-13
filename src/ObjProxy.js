import {Point as P} from 'phaser'

import Obj from './Obj'
import NwPosVelComponent from './NwPosVel'
import ScaleComponent from './Scale'
import SprComponent from './Spr'
import TextSprComponent from './TextSpr'

export default class PlayerProxy extends Obj {
  constructor (init) {
    let {game, pos, group, name} = init

    super({type: 'player', ...init})

    this.comps = {
      scale: new ScaleComponent(1),
      posVel: new NwPosVelComponent(init),
      spr: new SprComponent({
        game, pos, group, asset: 'ms'
      }),
      text: new TextSprComponent({game, pos, text: name})
    }

    this.ev('start')
  }

  onNetwork (init) {
    this.comps.posVel = new NwPosVelComponent(init)
  }

  update (time) {
    let {posVel, scale, spr, text} = this.comps

    let pos = posVel.update(time)

    scale.setFlipX(posVel.vel.x < 0)
    scale.update()

    spr.setScale(scale.getScale())
    spr.setPos(pos)
    spr.update()

    text.setPos(new P(pos.x, pos.y - 10))
    text.update()
  }
}
