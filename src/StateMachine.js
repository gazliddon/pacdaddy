function capitalize (s) {
  return s[0].toUpperCase() + s.slice(1)
}

export default class StateMachine {
  constructor (table, initState, target, onStateChange) {
    this.table = table
    this.state = initState
    const stateToFunc = { }

    this.target = target
    this.stateToFunc = stateToFunc

    this.onStateChange = onStateChange
  }

  getBoundFunc (name) {
    // Get a func for this event
    // and cache it
    let func = this.stateToFunc[name]

    if (!func) {
      let funcName = 'on' + capitalize(name)
      func = this.target[funcName]

      if (func) {
        func = func.bind(this.target)
        this.stateToFunc[name] = func
      }
    }

    return func
  }

  ev (ev, payLoad) {
    let evTab = this.table[ev]

    if (evTab) {
      let nextState = evTab[this.state]
      if (nextState) {
        let oldState = this.state
        this.state = nextState

        let func = this.getBoundFunc(nextState)

        if (this.onStateChange) {
          this.onStateChange(nextState, ev, oldState)
        }

        if (func) {
          let didChange = (nextState !== oldState)
          func(payLoad, didChange, ev, nextState, oldState)
        }
      }
    }
  }
}
