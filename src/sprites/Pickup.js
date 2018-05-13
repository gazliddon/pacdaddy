import {Sprite as Spr, Point as P} from 'phaser'
import StateMachine from '../StateMachine'
import TextSprComponent from '../TextSpr'

const table = {
  start: { nothing: 'idle' },
  localeat: { idle: 'localyEat' },
  eat: { localeat: 'die' }
}

// class NetworkObj {
//     constructor(id) {
//     }

//     die(id) {
//     }
// }

export default class extends Spr {
  constructor ({ game, pos, asset, kind, nwId, group, name }) {
    super(game, pos.x, pos.y, asset)

    this.pos = pos

    this.animations.add('coke', [3 * 14 + 2], 10, true)
    this.animations.add('pizza', [3 * 14 + 3], 10, true)
    this.animations.add('burger', [3 * 14 + 4], 10, true)

    this.setAnim(kind)

    this.sm = new StateMachine(table, 'nothing', this)
    this.sm.ev('start')

    this.anchor.set(0.5)
    this.scale.set(1)
    this.smoothed = false

    game.physics.arcade.enable(this)

    this.enableBody = true
    this.body.immovable = true

    if (name) {
      this.addName(name)
    }

    console.log(name)

    group.add(this)
  }

  kill () {
    super.kill()
    if (this.name) {
      this.name.kill()
    }
  }

  addName (name) {
    let {game, pos} = this
    let nameComp = new TextSprComponent({game, pos, text: name})
    this.name = nameComp
  }

  updateName () {
    let {name, pos} = this
    if (name) {
      name.setPos(new P(pos.x, pos.y - 10))
      name.update()
    }
  }

  onStart () {
  }

  onLocalyEat () {
    this.body.enable = false
  }

  onEat () {
  }

  onDie () {
  }

  update () {
    this.x = this.pos.x
    this.y = this.pos.y
    this.updateName()
  }

  setAnim (kind) {
    if (kind !== this.kind) {
      this.animations.play(kind)
      this.kind = kind
    }
  }
}
