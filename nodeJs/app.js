const express = require('express');
const http = require('http');
const pino = require('pino');
const expressPino = require('express-pino-logger');
const cookieParser = require('cookie-parser');
const delay = require('delay');
const {createLightship,} = require('lightship');

const mongo = require('./libs/mongo');
const graphql = require('./libs/graphql');

const config = require('./config')[process.env.PROFILE || 'dev'];

const logger = pino({
  level: process.env.LOG_LEVEL || 'info',
  prettyPrint: {
    ignore: 'pid,hostname',
    translateTime: 'SYS:standard',
    colorize: !process.env.PROFILE || process.env.PROFILE === 'dev'
  }
});
const expressLogger = expressPino({logger});

global.logger = logger;

const app = express();
const lightship = createLightship();
app.use('/graphql', graphql);

app.use(expressLogger);
app.use(express.json());
app.use(express.urlencoded({extended: false}));
app.use(cookieParser());

const port = normalizePort(process.env.PORT || '3000');
app.set('port', port);

const server = http.createServer(app);

server.on('error', onError);
server.on('listening', onListening);

const runningServer = server.listen(port);

lightship.registerShutdownHandler(async () => {
  // Allow sufficient amount of time to allow all of the existing
  // HTTP requests to finish before terminating the service.
  await delay(60 * 1000);

  runningServer.close();
});

mongo(config.mongo, lightship)

/**
 * Normalize a port into a number, string, or false.
 */

function normalizePort(val) {
  const port = parseInt(val, 10);

  if (isNaN(port)) {
    // named pipe
    return val;
  }

  if (port >= 0) {
    // port number
    return port;
  }

  return false;
}

/**
 * Event listener for HTTP server "error" event.
 */

function onError(error) {
  if (error.syscall !== 'listen') {
    throw error;
  }

  const bind = typeof port === 'string'
    ? 'Pipe ' + port
    : 'Port ' + port;

  // handle specific listen errors with friendly messages
  switch (error.code) {
    case 'EACCES':
      logger.error(bind + ' requires elevated privileges');
      process.exit(1);
      break;
    case 'EADDRINUSE':
      logger.error(bind + ' is already in use');
      process.exit(1);
      break;
    default:
      throw error;
  }
}

/**
 * Event listener for HTTP server "listening" event.
 */

function onListening() {
  const addr = server.address();
  const bind = typeof addr === 'string'
    ? 'pipe ' + addr
    : 'port ' + addr.port;
  logger.debug('Listening on ' + bind);
}
