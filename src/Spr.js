import {Sprite as Spr} from 'phaser'

export default class SprComponent {
  constructor ({ game, pos, asset, group }) {
    let spr = new Spr(game, pos.x, pos.y, asset)

    this.pos = pos
    this.game = game
    this.spr = spr

    spr.animations.add('wacca', [0, 1, 2, 1], 10, true)
    spr.animations.add('stopped', [0], 10, false)
    spr.anchor.set(0.5)
    spr.scale.set(1)
    spr.smoothed = false

    game.physics.arcade.enable(spr)
    spr.enableBody = true
    spr.body.immovable = true

    group.add(spr)
  }

  playAnim (name) {
    this.spr.animations.play(name)
  }

  setScale (v) {
    this.spr.scale.set(v.x, v.y)
  }

  setPos (v) {
    this.pos.x = v.x
    this.pos.y = v.y
  }

  update () {
    this.spr.x = this.pos.x
    this.spr.y = this.pos.y
  }
}
