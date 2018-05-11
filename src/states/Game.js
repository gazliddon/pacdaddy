import Phaser, { Point as P} from 'phaser'
import Pickup from '../sprites/Pickup'
import Socket from '../Socket'

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
    let game = this.game

    game.load.image('star', 'assets/images/star.png')
    this.load.spritesheet('ms', 'assets/images/pacman.png', 16, 16)
    this.stage.disableVisibilityChange = true
    this.scale.scaleMode = Phaser.ScaleManager.SHOW_ALL
    this.scale.pageAlignHorizontally = true
    this.scale.pageAlignVertically = true
    // this.scale.setScreenSize( true );
    // Phaser.Canvas.setSmoothingEnabled(this.game.context, false);
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

  makeStars (n) {
    let {game} = this
    let starsBatch = game.add.spriteBatch()
    let stars = []

    for (let i = 0; i < n; i++) {
      let star = game.make.sprite(Math.random() * 1920, Math.random() * 1920, 'star')
      star.anchor.set(0.5)
      starsBatch.addChild(star)
      stars.push(star)
    }

    this.stars = stars
  }

  create () {
    const game = this.game

    game.world.setBounds(0, 0, 1920, 1920)
    game.camera.scale.setTo(2, 2)

    this.makeStars(1000)
    this.makeBanner()

    const onScreenItems = new OnScreenItems(game)
    const gameState = new GameState(game)

    const onMessage = (res, incoming) => {
      gameState.dispatch(incoming.msg, res, incoming)
    }

    const onOpen = (res) => {
      res.send({ msg: 'hello', time: 0, id: 0 })
      res.flush()
    }

    console.log(location)

    const socket = new Socket(location.hostname, 6502, {onMessage, onOpen})

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
    let {socket, gameState, onScreenItems} = this

    onScreenItems.adjust(gameState.objs, this.id)
    gameState.update(socket)
    onScreenItems.update()
  }

  render () {
  }
}
