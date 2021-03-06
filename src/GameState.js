import Player from './Player'
import Phaser, {Point as P} from 'phaser'
import Dispatchable from './Dispatchable'
import _ from 'lodash'

export default class extends Dispatchable {
  constructor (game, id) {
    super()
    this.id = id
    this.game = game
    this.player = null
    this.players = {}
    this.objs = {}
    this.time = -1
    this.group = game.add.group()
    this.sfx = game.add.audio('eat_dot')
    // Debug
    window.gs = this
  }

  networkTime (time) {
  }

  onPlayerInfo (res, {data, time}) {
    let {id} = data
    // is this another player
    if (id !== this.id) {
      // Todo create or update a proxy for this player
    }
  }

  onPlayerJoined (res, {data, time}) {
    let {game, group, id} = this
    let {pos: {x, y}, name} = data
    this.time = time
    this.lastTime = performance.now()
    const pos = new P(x, y)
    const max = new P(1920, 1920)
    this.player = new Player({game, pos, id, group, name, max})
  }

  mkMessage (msg, data) {
    let {id, time} = this
    return { msg, id, time, data }
  }

  sendNow (res, msg, data) {
    const reply = this.mkMessage(msg, data)
    res.sendNow(reply)
  }

  onPlayer (res, payload) {
  }

  onDelete (res, {data: {toDelete}}) {
    this.removeSpr(toDelete)
  }

  onPlayerDelete (res, {data}) {
    let {toDelete} = data
    delete this.players[toDelete]
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

  onPickupInfo (res, {data}) {
    this.objs[data.uuid] = data
  }

  onState (res, payLoad) {
    let {id} = this
    let {data: {pickups, players}} = payLoad

    let newPlayers = {}

    _.forEach(players, p => {
      if (p.uuid !== id) {
        newPlayers[p.uuid] = p
      }
    })

    this.players = newPlayers

    let newPickups = {}

    _.forEach(pickups, o => {
      newPickups[o.uuid] = o
    })

    this.objs = newPickups
  }

  getPlayers () {
    return _.map(this.players, ({name, score}, k) => {
      return {name, score}
    })
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

      this.sendNow(res, 'playerUpdate', payLoad)
    } else {
      game.camera.follow()
    }
  }
}
