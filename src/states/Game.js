import Phaser, { Color, Point as P} from 'phaser'
import Pickup from '../sprites/Pickup'

import GameState from '../GameState'

import _ from 'lodash'

import {updatePlayersPanel} from '../Panel'

class OnScreenItems {
  constructor (game) {
    this.game = game
    this.objs = {}
    this.group = game.add.group()
    this.asset = 'ms'
    this.names = {}
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

  addSpr ({name, id: nwId, pos: {x, y}, kind}) {
    const {game, group, asset} = this
    const pos = new P(x, y)
    const myObj = new Pickup({ game, kind, group, pos, asset, nwId, name })
    this.objs[nwId] = myObj
  }

  updateSpr (obj, {pos: {x, y}, kind}) {
    obj.pos = new P(x, y)
    obj.setAnim(kind)
  }

  removeSpr (id) {
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
    sprObj.t = this.game.rnd.frac() * sprObj.maxT
    sprObj.scale = 0.5 + this.game.rnd.frac() * 0.5
  }

  makeStars (n) {
    let {game} = this

    let stars = []

    let batch = game.add.spriteBatch()

    for (let i = 0; i < n; i++) {
      let star = game.make.sprite(0, 0, 'star')
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

      let dt = 1 - Math.abs(Math.cos((t / maxT) * 2 * 3.141592))

      spr.scale.set(2 + dt, 2 + dt)

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

  exit () {
    this.music.fadeOut(100)
    this.state.start('Splash')
  }

  create () {
    let {game} = this
    let {socket, id, name} = window.billboard

    let music = game.add.audio('main')

    music.volume = 0.5
    music.loop = true
    music.play()
    this.music = music

    this.scaleTarget = 2.4
    this.scale = 0.1
    this.scaleLerp = 100

    game.world.setBounds(0, 0, 1920, 1920)

    this.makeStars(1000)
    this.makeBanner()

    const onScreenItems = new OnScreenItems(game)
    const gameState = new GameState(game)

    this.socket = socket
    this.id = id
    this.name = name

    socket.onMessage((res, incoming) => {
      console.log(incoming.msg)
      gameState.dispatch(incoming.msg, res, incoming)
    })

    socket.onClose((incoming) => {
      this.exit()
    })

    socket.sendNow({ msg: 'hello', time: 0, id: id, data: {name: name} })

    const updateCounter = () => {
      socket.flush()
    }

    this.gameState = gameState
    this.onScreenItems = onScreenItems
    this.socket = socket
    game.physics.startSystem(Phaser.Physics.ARCADE)
    this.time.events.loop(Phaser.Timer.SECOND * 3, updateCounter, this)
    // const pos = new P(this.world.centerX, this.world.centerY)
    //
    //
  }

  getMousePosWorld () {
    let {game} = this

    let sx = game.camera.scale.x
    let sy = game.camera.scale.y
    let x = game.input.activePointer.worldX / sx
    let y = game.input.activePointer.worldY / sy

    return new P(x, y)
  }

  update () {
    let {game, socket, gameState, onScreenItems} = this

    let b = (game.camera.y / 1920)

    if (b > 1) { b = 1 }

    b = Math.floor(b * 255)
    const c = Color.RGBtoString(0, 0, b, 255, '#')
    game.stage.backgroundColor = c

    onScreenItems.adjust(gameState.objs, this.id)
    gameState.update(socket)
    onScreenItems.update()
    let camPos = new P(game.camera.x, game.camera.y)
    this.updateStars(camPos)
  }

  render () {
    let {gameState, game, scale, scaleTarget, scaleLerp} = this

    scale = scale + (scaleTarget - scale) / scaleLerp

    scaleTarget = 1 / (gameState.getPlayerScale() / 5.0)

    if (scaleTarget < 0.2) {
      scaleTarget = 0.2
    }

    if (scaleTarget > 3) {
      scaleTarget = 3
    }

    this.scale = scale
    this.scaleTarget = scaleTarget

    game.camera.scale.setTo(this.scale)

    let players = gameState.getPlayers()
    updatePlayersPanel(players)
  }
}
