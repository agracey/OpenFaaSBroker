const express = require('express')
const app = express()
const api = require('./api/index.js')
const winston = require('winston')

// set up default logging https://github.com/winstonjs/winston#using-the-default-logger
winston.add(winston.createLogger({
  format: winston.format.combine(
    winston.format.colorize(),
    winston.format.splat(),
    winston.format.simple()
  ),
  transports: [new winston.transports.Console()]
}))

app.use(express.json())
app.use('/v2', api)

app.listen(process.env.PORT || 8080, (err) => {
  if (err) {
    winston.error(err)
    return
  }

  winston.warn('Listening on port %s', process.env.PORT || 8080)
})
