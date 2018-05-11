import {Point as P, Sprite as Spr} from 'phaser'

export default class extends Spr {
    constructor ({ game, pos, vel, asset, fric }) {
        super(game, pos.x, pos.y, asset)
        this.anchor.setTo(0.5)
        this.pos = pos
        this.vel = vel
        this.fric = fric
    }

    update () {
        this.pos = P.add(this.pos, this.vel)
        P.add(this.pos, this.vel, this.pos)
        P.multiply(this.vel, this.fric, this.vel)
        this.x = this.pos.x
        this.y = this.pos.y
    }
}
