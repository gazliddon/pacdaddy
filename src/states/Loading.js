import Phaser from 'phaser'

export default class extends Phaser.State {
  create () {
    let {game} = this

    // You can listen for each of these events from Phaser.Loader
    game.load.onLoadStart.add(this.loadStart, this)
    game.load.onFileComplete.add(this.fileComplete, this)
    game.load.onLoadComplete.add(this.loadComplete, this)

    let button = game.add.button(game.world.centerX - 95, 400, 'button', this.start, this, 2, 1, 0)
    let text = game.add.text(32, 32, 'Click to start load', { fill: '#ffffff' })

    this.button = button
    this.text = text
  }

  preload () {
  }

  loadComplete () {
    this.text.setText('Load Complete')
    this.state.start('Splash')
  }

  fileComplete (progress, cacheKey, success, totalLoaded, totalFiles) {
    let {text} = this
    text.setText('File Complete: ' + progress + '% - ' + totalLoaded + ' out of ' + totalFiles)
  }

  start () {
    let {game} = this

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

    game.load.start()
    this.button.visible = false
  }

  loadStart () {
    this.text.setText('Loading ...')
  }
}
