export default class Logger {
  static timeStampString() {
    const currentTime = new Date();
    return `${currentTime.getFullYear()}/${
      currentTime.getMonth() + 1
    }/${currentTime.getDate()} - ${currentTime.getHours()}:${currentTime.getMinutes()}:${currentTime.getSeconds()}`;
  }
  static Info(data) {
    const log = `[INFO] - ${this.timeStampString()} - ${data}`;
    console.log(log);
    return log;
  }
  static Warn(data) {
    const log = `[WARN] - ${this.timeStampString()} - ${data}`;
    console.warn(log);
    return log;
  }
  static Error(data) {
    const log = `[ERROR] - ${this.timeStampString()} - ${data}`;
    console.error(log);
    return log;
  }
}
