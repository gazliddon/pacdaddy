import _ from 'lodash'

function sleep (ms) {
  return new Promise(resolve => setTimeout(resolve, ms))
}

export class Socket {
  constructor (socket) {
    this.q = []
    this.socket = socket
  }

  onMessage (msgFunc) {
    this.socket.onmessage = (event) => {
      let data = JSON.parse(event.data)
      msgFunc(this, data)
    }
  }

  onClose (closeFunc) {
    this.socket.onclose = (event) => {
      closeFunc(event)
    }
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

function makeSocket (url, port) {
  let uri = 'ws://' + url + ':' + port
  return new Promise((resolve, reject) => {
    let server = new WebSocket(uri)

    server.onopen = () => {
      resolve(new Socket(server))
    }

    server.onerror = (err) => {
      reject(err)
    }
  })
}

export async function makeSocketRetry (hostName, port, retryTime, onNoConnect) {
  let trys = 0

  while (1) {
    try {
      return await makeSocket(hostName, port)
    } catch (err) {
      if (onNoConnect) {
        onNoConnect(err, trys)
      }
      await sleep(retryTime)
      trys = trys + 1
    }
  }
}
