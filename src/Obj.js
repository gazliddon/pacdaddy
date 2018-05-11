import StateMachine from './StateMachine'

export default class Obj {
  constructor ({game, type, id, table}) {
    this.id = id
    this.type = type
    this.game = game
    this.sm = new StateMachine(table, 'nothing', this)
    this.comps = []
  }

  ev (ev, payload) {
    this.sm.ev(ev, payload)
  }
}
