import Phaser from 'phaser'
// import {centerGameObjects} from '../utils'
import {makeSocketRetry} from '../Socket'
import StateMachine from '../StateMachine'
import _ from 'lodash'

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
    // let resources = {
    //   images: {
    //     title: 'assets/images/title.png',
    //     mushroom: 'assets/images/mushroom2.png',
    //     star: 'assets/images/star.png'
    //   },

    //   audio: {
    //     main: 'assets/zone of endor.mp3',
    //     title: 'assets/galax.mp3'
    //   }
    // }

    this.load.image('title', 'assets/images/title.png')
    this.load.image('mushroom', 'assets/images/mushroom2.png')
    this.load.image('star', 'assets/images/star.png')
    this.load.audio('main', 'assets/audio/zone of endor.mp3')
    this.load.audio('title', 'assets/audio/galax.mp3')

    this.load.audio('eat_dot', 'assets/audio/eat_dot.wav')
    this.load.audio('eat_ghost', 'assets/audio/eat_ghost.wav')
    this.load.audio('eyes_running_sound', 'assets/audio/eyes_running_sound.wav')
    this.load.audio('normal_fast_siren', 'assets/audio/normal_fast_siren.wav')
    this.load.audio('normal_slow_siren', 'assets/audio/normal_slow_siren.wav')
    this.load.audio('player_die', 'assets/audio/player_die.wav')
    this.load.audio('running_ghost', 'assets/audio/running_ghost.wav')

    this.load.spritesheet('ms', 'assets/images/pacman.png', 16, 16)
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
          let obj = {
            socket, id, name: window.localStorage.name
          }
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
    this.music.fadeOut(1000)
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

    banner.padding.set(10, 16)
    banner.anchor.setTo(0.5)
    banner.inputEnabled = true

    banner.events.onInputDown.add(() => {
      this.sm.ev(ev)
    })

    banner.events.onInputOver.add(() => {
      banner.fill = '#eee'
    }, this)

    banner.events.onInputOut.add(() => {
      banner.fill = '#aaa'
    }, this)
  }

  mkMenu (table, x, y) {
    return _.map(table, ({text, ev}) => {
      let m = this.mkMenuItem(x, y, text, ev)
      y = y + 80
      return m
    })
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
