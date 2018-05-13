import Player from './Player'
import Phaser, {Point as P} from 'phaser'
import Dispatchable from './Dispatchable'
import _ from 'lodash'

export default class extends Dispatchable {
  constructor (game) {
    super()
    this.game = game
    this.player = null
    this.players = {}
    this.objs = {}
    this.id = -1
    this.time = -1
    this.group = game.add.group()

    this.sfx = game.add.audio('eat_dot')
  }

  networkTime (time) {
  }

  onJoined (res, {data, time}) {
    let {game, group} = this
    let {id, pos: {x, y}, name} = data
    this.id = id
    this.time = time
    this.lastTime = performance.now()
    const pos = new P(x, y)
    const max = new P(1920, 1920)
    this.player = new Player({game, pos, id, group, name, max})
  }

  mkMessage (msg, data) {
    return {
      msg,
      data,
      time: this.time,
      id: this.id
    }
  }

  sendNow (res, msg, data) {
    const reply = this.mkMessage(msg, data)
    res.sendNow(reply)
  }

  onPlayer (res, payload) {
  }

  onPing (res, incoming) {
    let {time, data: {id}} = incoming
    this.sendNow(res, 'pong', {id, time})
  }

  removeSpr (id) {
    let spr = this.objs[id]
    if (spr) {
      delete this.objs[id]
      spr.kill()
    }
  }

  onEatFruit (res, {id}) {
    this.removeSpr(id)
    this.sfx.play()
  }

  onPlayerUpdate (res, payLoad) {
    let {pos, vel, id} = payLoad.data

    let o = this.objs[id]

    if (o) {
      o.pos = pos
      o.vel = vel
    } else {
    }
  }

  onState (res, payLoad) {
    let {data: {objs, players}} = payLoad

    this.players = players

    let newObjs = {}

    _.forEach(objs, o => {
      if (o.id !== this.id) {
        newObjs[o.id] = o
      }
    })

    this.objs = newObjs
  }

  update (res) {
    let {player, game} = this

    _.forEach(this.objs, (v, k) => {
      let {pos, vel} = v

      if (vel) {
        v.pos = P.add(pos, vel)
      }
    })

    if (player) {
      player.update()

      let pspr = player.getSprite()

      game.camera.follow(pspr, Phaser.Camera.FOLLOW_LOCKON, 0.1, 0.1)

      let payLoad = {
        pos: {
          x: player.comps.posVel.pos.x,
          y: player.comps.posVel.pos.y
        },
        vel: {
          x: player.comps.posVel.vel.x,
          y: player.comps.posVel.vel.y
        }
      }

      this.sendNow(res, 'player', payLoad)
    } else {
      game.camera.follow()
    }
  }
}
