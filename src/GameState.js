import Player from './Player'
import {Point as P} from 'phaser'
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
  }

  networkTime (time) {
  }

  onJoined (res, {data, time}) {
    let {game, group} = this
    let {id, pos: {x, y}} = data
    this.id = id
    this.time = time
    this.lastTime = performance.now()
    const pos = new P(x, y)
    this.player = new Player({game, pos, id, group})
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
    let {player} = this

    if (player) {
      player.update()

      let payLoad = {
        pos: {
          x: player.comps.posVel.pos.x,
          y: player.comps.posVel.pos.y
        }
      }

      this.sendNow(res, 'player', payLoad)
    }
  }
}
