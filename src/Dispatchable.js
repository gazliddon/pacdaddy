function capitalize (s) {
  return s[0].toUpperCase() + s.slice(1)
}

export default class {
  constructor () {
    this._stateToFunc = {}
  }

  getBoundFunc (msg) {
    let func = this._stateToFunc[msg]

    if (!func) {
      let funcName = 'on' + capitalize(msg)
      func = this[funcName]

      if (func) {
        func = func.bind(this)
        this._stateToFunc[msg] = func
      }
    }
    return func
  }

  dispatch (msg, ...theArgs) {
    let fn = this.getBoundFunc(msg)
    if (fn) {
      fn(...theArgs)
      return true
    } else {
      return false
    }
  }
}
