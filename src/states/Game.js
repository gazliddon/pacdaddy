import Phaser, { Point as P} from 'phaser'
import Pickup from '../sprites/Pickup'
import {makeSocket} from '../Socket'

import GameState from '../GameState'

import _ from 'lodash'

class OnScreenItems {
  constructor (game) {
    this.game = game
    this.objs = {}
    this.group = game.add.group()
    this.asset = 'ms'
  }

  adjust (gameStateObjs) {
    const gsObjIds = _.keys(gameStateObjs)
    const myObjIds = _.keys(this.objs)

    // Update information on ids we have
    _.intersection(gsObjIds, myObjIds).forEach(id => {
      this.updateSpr(this.objs[id], gameStateObjs[id])
    })

    // Remove the sprs we don't have
    _.difference(myObjIds, gsObjIds).forEach(id => {
      this.removeSpr(id)
    })

    // Add any new sprs
    _.difference(gsObjIds, myObjIds).forEach(id => {
      this.addSpr(gameStateObjs[id])
    })
  }

  addSpr ({id: nwId, pos: {x, y}, kind}) {
    const {game, group, asset} = this
    const pos = new P(x, y)
    const myObj = new Pickup({ game, kind, group, pos, asset, nwId })
    this.objs[nwId] = myObj
  }

  updateSpr (obj, {pos: {x, y}, kind}) {
    obj.pos = new P(x, y)
    obj.setAnim(kind)
  }

  removeSpr (id) {
    console.log(id)
    let spr = this.objs[id]
    delete this.objs[id]
    spr.kill()
  }

  update () {
    _.forEach(this.objs, (v, k) => {
      v.update()
    })
  }
}

export default class extends Phaser.State {
  init () {
    this.stage.backgroundColor = '#222'
    this.networkobjs = {}
  }

  preload () {
    // let game = this.game

    this.stage.disableVisibilityChange = true
    this.scale.scaleMode = Phaser.ScaleManager.SHOW_ALL
    this.scale.pageAlignHorizontally = true
    this.scale.pageAlignVertically = true
  }

  makeBanner () {
    let {game, world} = this
    const bannerText = 'pacdaddy'

    let banner = this.add.text(world.centerX, game.height - 80, bannerText, {
      font: '32px Arial',
      fill: '#77BFA3',
      smoothed: false
    })

    banner.padding.set(10, 16)
    banner.anchor.setTo(0.5)
  }

  setupStar (sprObj) {
    let {spr} = sprObj
    let p = new P(Math.random() * 1920, Math.random() * 1920)
    spr.x = p.x
    spr.y = p.y
    sprObj.pos = p
    sprObj.maxT = this.game.rnd.frac() * 120 + 360
    sprObj.t = 0
    sprObj.scale = 0.5 + this.game.rnd.frac() * 0.5
  }

  makeStars (n) {
    let {game} = this

    let stars = []

    let batch = game.add.spriteBatch()

    for (let i = 0; i < n; i++) {
      let p = new P(Math.random() * 1000, Math.random() * 1000)

      let star = game.make.sprite(p.x, p.y, 'star')
      star.anchor.set(0.5)
      star.fixedToCamera = true

      let starObj = { spr: star }
      this.setupStar(starObj)

      stars.push(starObj)
      batch.addChild(star)
    }

    this.stars = stars
    this.starsBatch = batch
  }

  updateStars ({x, y}) {
    for (const sprObj of this.stars) {
      let {scale, spr, t, maxT} = sprObj

      let dt = 1 - Math.abs(Math.cos( (t / maxT) * 2 * 3.141592))

      spr.alpha = dt

      if (t > maxT) {
        this.setupStar(sprObj)
      } else {
        sprObj.t = t + 1
      }

      let {pos} = sprObj

      spr.cameraOffset.x = pos.x - x * scale
      spr.cameraOffset.y = pos.y - y * scale
    }
  }

  create () {
    let {socket} = window.billboard

    const game = this.game

    game.world.setBounds(0, 0, 1920, 1920)
    game.camera.scale.setTo(4, 4)

    this.makeStars(500)
    this.makeBanner()

    const onScreenItems = new OnScreenItems(game)
    const gameState = new GameState(game)

    this.socket = socket

    socket.onMessage((res, incoming) => {
      gameState.dispatch(incoming.msg, res, incoming)
    })

    socket.onClose((incoming) => {
      this.state.start('Splash')
    })

    console.log(window.billboard)

    socket.sendNow({ msg: 'hello', time: 0, id: 0, data: {name: window.billboard.name} })

    const updateCounter = () => {
      socket.flush()
    }

    this.gameState = gameState
    this.onScreenItems = onScreenItems
    this.socket = socket
    game.physics.startSystem(Phaser.Physics.ARCADE)
    this.time.events.loop(Phaser.Timer.SECOND * 3, updateCounter, this)
    // const pos = new P(this.world.centerX, this.world.centerY)
  }

  update () {
    let {game, socket, gameState, onScreenItems} = this
    onScreenItems.adjust(gameState.objs, this.id)
    gameState.update(socket)
    onScreenItems.update()

    let camPos = new P(game.camera.x, game.camera.y)
    this.updateStars(camPos)
  }

  render () {
  }
}
