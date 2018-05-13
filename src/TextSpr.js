
export default class TextSprComponent {
  constructor ({ game, pos, text }) {
    let style = { font: '10px Arial', fill: '#fff', boundsAlignH: 'center', boundsAlignV: 'middle' }

    const textSpr = game.add.text(pos.x, pos.y, text, style)

    this.pos = pos
    this.game = game
    this.textSpr = textSpr
    textSpr.anchor.set(0.5)
    textSpr.scale.set(1)
    textSpr.smoothed = false
  }

  setPos (v) {
    this.pos.x = v.x
    this.pos.y = v.y
  }

  kill () {
    this.textSpr.kill()
  }

  update () {
    this.textSpr.x = this.pos.x
    this.textSpr.y = this.pos.y
  }
}
