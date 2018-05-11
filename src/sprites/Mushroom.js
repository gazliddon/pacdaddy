import {Point as P, Sprite as Spr} from 'phaser'
import StateMachine from '../StateMachine'

const table = {
    ev_start: { nothing: 'idle' },
    ev_stickmove: { idle: 'move', slide: 'move' },
    ev_stickidle: { move: 'slide' },
    ev_stopped: { slide: 'idle', move: 'idle' }
}

export default class extends Spr {
    constructor ({ game, pos, vel, asset, fric }) {
        super(game, pos.x, pos.y, asset)
        this.pos = pos
        this.vel = vel
        this.fric = new P(1, 1)
        this.fric_config = fric

        this.my_scale = 1

        this.animations.add('wacca', [0, 1, 2, 1], 10, true)
        this.animations.add('coke', [3 * 14 + 2], 10, true)
        this.animations.add('pizza', [3 * 14 + 3], 10, true)
        this.animations.add('burger', [3 * 14 + 4], 10, true)

        this.sm = new StateMachine(table, 'nothing', this)
        this.sm.ev('ev_start')

        this.anchor.set(0.5)
        this.scale.set(1)
        this.smoothed = false

        game.physics.arcade.enable(this)
        this.enableBody = true
        this.body.immovable = true
    }

    enbiggen () {
        this.my_scale *= 1.1
    }

    onStart () {
        this.fric = new P(1, 1)
    }

    onSlide () {
        this.vel.set(0, 0)
        this.fric = new P(0.9, 0.9)
    }

    onIdle () {
        this.animations.stop()
    }

    onMove (v) {
        this.fric = new P(1, 1)
        if (v.x > 0.0001) {
            this.scale.set(1 * v.x, 1)
        }
        this.animations.play('wacca')
    }

    handleStick (vel) {
        const maxVel = 1.5
        const velScale = new P(0.9, 0.9)

        P.multiply(vel, velScale, vel)

        let newVel = new P()
        P.add(vel, this.vel, newVel)

        if (newVel.getMagnitude() > maxVel) {
            newVel.setMagnitude(maxVel)
        }

        this.vel = newVel

        if (vel.getMagnitude() < 0.0000001) {
            this.sm.ev('ev_stickidle')
        } else {
            this.sm.ev('ev_stickmove', vel)
        }

        if (newVel.getMagnitude() < 0.1) {
            this.sm.ev('ev_stopped')
        }
    }

    update () {
        this.pos = P.add(this.pos, this.vel)
        P.add(this.pos, this.vel, this.pos)
        P.multiply(this.vel, this.fric, this.vel)
        this.x = this.pos.x
        this.y = this.pos.y

        if (this.vel.x < 0) {
            this.scale.set(-1 * this.my_scale, 1 * this.my_scale)
        } else {
            this.scale.set(1 * this.my_scale, 1 * this.my_scale)
        }
    }
}
