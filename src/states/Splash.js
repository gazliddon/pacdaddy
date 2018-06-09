import Phaser from 'phaser'
// import {centerGameObjects} from '../utils'
import {makeSocketRetry} from '../Socket'
import StateMachine from '../StateMachine'
import _ from 'lodash'
import Clock from '../Clock'

const table = {
  start: { nothing: 'clickMe' },
  changeName: {clickMe: 'changeName'},
  playGame: { clickMe: 'connecting' },
  done: {changeName: 'clickMe'},
  retry: { connecting: 'retry', retry: 'retry' },
  connected: { connecting: 'fading', retry: 'fading' },
  fadeComplete: { fading: 'startGame' },
  disconnect: {connecting: 'error'}
}

export default class extends Phaser.State {
  init () {
    let {game} = this
    this.stage.backgroundColor = '#660f0f'
    window.billboard = {}
    game.camera.scale.setTo(1, 1)
    this.sm = new StateMachine(table, 'nothing', this)
  }

  preload () {
  }

  onClickMe () {
  }

  onChangeName () {
    let ls = window.localStorage
    let person = prompt('Please enter your name', ls.name)
    ls.name = person
    this.sm.ev('done')
  }

  onError () {
    console.log('connection error')
  }

  onConnecting () {
    const onRetry = (_, trys) => {
      this.sm.ev('retry', trys)
    }

    const onConnect = (socket) => {
      socket.onMessage((s, {id, time, msg}) => {
        if (msg === 'madeConnection') {
          let clock = new Clock(time)

          let obj = {
            socket,
            id,
            clock,
            name: window.localStorage.name
          }
          console.log(obj)
          this.sm.ev('connected', obj)
        } else {
          this.sm.ev('disconnect')
        }
      })
    }

    const onError = (err) => {
      console.log('this is an error')
      console.log(err)
    }

    makeSocketRetry(location.hostname, 6502, 1000, onRetry)
      .then(onConnect)
      .catch(onError)
  }

  onConnected () {
  }

  onRetry (tries) {
  }

  onFading (obj) {
    this.music.fadeOut(500)
    this.camera.fade('#000000')
    this.camera.onFadeComplete.add(() => {
      this.sm.ev('fadeComplete', obj)
    }, this)
  }

  onStartGame (obj) {
    window.billboard = obj
    this.music.stop()
    this.state.start('Game')
  }

  makeTitle () {
    let {game, world} = this
    let title = game.add.sprite(world.centerX, world.centerY - 200, 'title')
    title.anchor.set(0.5)
    title.smoothed = false
    return title
  }

  mkMenuItem (x, y, text, ev) {
    let {game} = this

    let banner = game.add.text(x, y, text, {
      font: '60px Bungee Shade',
      fill: '#eee',
      smoothed: false
    })

    banner.fill = '#a0a0a0'

    banner.padding.set(10, 16)
    banner.anchor.setTo(0.5)
    banner.inputEnabled = true

    banner.events.onInputDown.add(() => {
      this.sm.ev(ev)
    })

    banner.events.onInputOver.add(() => {
      // banner.fill = '#eee'
      this.flash = banner
    }, this)

    banner.events.onInputOut.add(() => {
      banner.backgroundColor = '#aaa'
      banner.fill = '#a0a0a0'
      this.flash = null
    }, this)
  }

  mkMenu (table, x, y) {
    return _.map(table, ({text, ev}) => {
      let m = this.mkMenuItem(x, y, text, ev)
      y = y + 80
      return m
    })
  }

  render () {
    let {flash} = this
    if (flash) {
      flash.fill = '#ffffff'
    }
  }

  create () {
    let {game, world} = this

    let music = this.add.audio('title')

    let table = [
      { text: 'PLAY', ev: 'playGame' },
      { text: 'CHANGE NAME', ev: 'changeName' }
    ]

    this.menu = this.mkMenu(table, world.centerX, game.height - 180)

    music.volume = 0.1
    music.loop = true
    music.play()
    this.music = music

    world.setBounds(0, 0, game.canvas.clientWidth, game.canvas.clientHeight)
    this.title = this.makeTitle()

    this.sm.ev('start')
  }
}
