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
    console.log(data)
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

  onEatFruit (res, {id}) {
    this.sfx.play()
    this.remove(id)
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

    if (player) {
      player.update()

      let pspr = player.getSprite()

      game.camera.follow(pspr, Phaser.Camera.FOLLOW_LOCKON, 0.1, 0.1)

      let payLoad = {
        pos: {
          x: player.comps.posVel.pos.x,
          y: player.comps.posVel.pos.y
        }
      }

      this.sendNow(res, 'player', payLoad)
    } else {
      game.camera.follow()
    }
  }
}
