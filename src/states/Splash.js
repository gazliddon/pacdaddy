import Phaser from 'phaser'
import {centerGameObjects} from '../utils'
import {makeSocketRetry} from '../Socket'
import StateMachine from '../StateMachine'

const table = {
  start: { nothing: 'clickMe' },
  click: { clickMe: 'connecting' },
  retry: { connecting: 'retry', retry: 'retry' },
  connected: { connecting: 'fading', retry: 'fading' },
  fadeComplete: { fading: 'startGame' }
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

  onConnecting () {
    let ls = window.localStorage
    let person = prompt('Please enter your name', ls.name)
    ls.name = person

    const onRetry = (_, trys) => {
      this.sm.ev('retry', trys)
    }

    const onConnect = (socket) => {
      this.sm.ev('connected', socket)
    }

    const onError = (err) => {
      this.banner.text = 'sad face'
      console.log('this is an error')
      console.log(err)
    }

    makeSocketRetry(location.hostname, 6502, 1000, onRetry)
      .then(onConnect)
      .catch(onError)
  }

  onRetry (tries) {
    this.banner.text = this.bannerText + ' ' + tries
  }

  onFading (socket) {
    this.music.fadeOut(1000)
    this.banner.text = 'connected!'
    this.camera.fade('#000000')
    this.camera.onFadeComplete.add(() => {
      this.sm.ev('fadeComplete', socket, window.localStorage.name)
    }, this)
  }

  onStartGame (socket, name) {
    this.music.stop()
    window.billboard.socket = socket
    window.billboard.name = window.localStorage.name
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
    let banner = this.add.text(x, y, text, {
      font: '80px Bungee Shade',
      fill: '#eee',
      smoothed: false
    })

    banner.padding.set(10, 16)
    banner.anchor.setTo(0.5)
    banner.inputEnabled = true
    banner.events.onInputDown.add(() => {
      this.sm.ev(ev)
    })
  }

  mkMenu (table, x, y) {
    let menu = []

    table.forEach(({text, ev}) => {
      menu.push(this.mkMenuItem(x, y, text, ev))
    })

    return menu
  }

  create () {
    let {game, world} = this

    let music = this.add.audio('title')

    this.bannerText = 'CLICK ME'

    let banner = this.add.text(world.centerX, game.height - 80, this.bannerText, {
      font: '80px Bungee Shade',
      fill: '#eee',
      smoothed: false
    })

    banner.padding.set(10, 16)
    banner.anchor.setTo(0.5)
    banner.inputEnabled = true
    this.banner = banner

    music.volume = 0.1
    music.loop = true
    music.play()
    this.music = music

    world.setBounds(0, 0, game.canvas.clientWidth, game.canvas.clientHeight)
    this.title = this.makeTitle()

    this.banner.events.onInputDown.add(() => {
      this.banner.inputEnabled = false
      this.sm.ev('click')
    })

    this.title.events.onInputOver.add(() => {
      // this.title.tint = Math.random() * 0xffffff
    })

    this.sm.ev('start')
  }
}
