import _ from 'lodash'

export default class Socket {
  constructor (url, port, {onOpen, onMessage}) {
    this.q = []

    let socket = new WebSocket('ws://' + url + ':' + port)

    if (onOpen) {
      socket.addEventListener('open', () => {
        onOpen(this)
      })
    }
    socket.addEventListener('message', (event) => {
      let data = JSON.parse(event.data)
      if (onMessage) {
        onMessage(this, data)
      }
    })

    this.socket = socket
  }

  send (item) {
    this.q.push(item)
  }

  sendNow (m) {
    this.socket.send(JSON.stringify(m))
  }

  flush () {
    _.forEach(this.q, (m) => {
      this.socket.send(JSON.stringify(m))
    })

    this.q = []
  }

  receive (item) {
    this.cb(JSON.parse(item))
  }
}
