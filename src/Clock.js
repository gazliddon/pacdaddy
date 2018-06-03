export default class {
  construct (time) {
    this.timeBase = performance.now()
    this.serverTimeBase = time / 1000000.0
  }
}
