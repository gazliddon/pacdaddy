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

    window.gs = this
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
    }
  }

  onEatFruit (res, { data: {id} }) {
    this.removeSpr(id)
    this.sfx.play()
  }

  onPlayerUpdate (res, payLoad) {
    let {pos, vel, id} = payLoad.data

    let o = this.objs[id]

    if (o) {
      o.serverPos = pos
      o.serverVel = vel
    }

    this.players[id] = payLoad.data
  }

  onState (res, payLoad) {
    let {id} = this
    let {data: {objs, players}} = payLoad

    let newPlayers = {}

    _.forEach(players, p => {
      newPlayers[p.id] = p
    })

    this.players = newPlayers

    let newObjs = {}

    _.forEach(objs, o => {
      if (o.id !== id) {
        newObjs[o.id] = o
      }
    })

    this.objs = newObjs
  }

  getPlayers () {
    let list = []

    _.forEach(this.players, (v, k) => {
      let {name, score} = v
      list.push({name, score})
    })

    return list
  }

  getPlayerScale () {
    let {player} = this

    if (player) {
      return player.getScale()
    } else {
      return 1
    }
  }

  update (res) {
    let {id, player, game, players} = this

    _.forEach(this.objs, (v, k) => {
      let {serverPos, serverVel} = v

      if (serverPos && serverVel) {
        serverPos = P.add(serverPos, serverVel)
        v.pos = serverPos
        v.serverPos = serverPos
      }
    })

    if (player) {
      if (players[id]) {
        player.score = players[id].score
        player.setScale(players[id].scale)
      }

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
