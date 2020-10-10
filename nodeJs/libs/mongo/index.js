const mongoose = require('mongoose');

module.exports = (config, lightship) => {
	const connection = mongoose.connection;

	const { MONGO_USER, MONGO_PASS } = process.env;
	const { port, hosts, 'admin-db': db, database, rsName, } = config;
	const connString = db
		? `mongodb://${MONGO_USER}:${MONGO_PASS}@${hosts.map(host => `${host}:${port}`).join(',')}/${database}?authSource=${db}${rsName ? `&replicaSet=${rsName}` : ''}`
		: `mongodb://${hosts.map(host => `${host}:${port}`).join(',')}/${database}`;

	connection.on('connecting', () => {
		logger.info('Connecting to MongoDB on port %d', port);
	});
	connection.on('error', error => {
		logger.error('Error in MongoDB connection: %O', error);
		mongoose.disconnect();
		lightship.signalNotReady()
		process.exit(1);
	});
	connection.on('connected', () => {
		logger.info('MongoDB connected! Listen to port %d', port);
		lightship.signalReady();
	});
	connection.once('open', () => {
		logger.info('MongoDB connection opened!');
	});
	connection.on('reconnected', () => {
		logger.info('MongoDB reconnected!');
		lightship.signalReady();
	});
	connection.on('disconnected', () => {
		logger.warn('MongoDB disconnected! %s', mongoose.connection.readyState);
		lightship.signalNotReady()
		process.exit(1);
	});

	mongoose.connect(connString, {
		readPreference: 'secondaryPreferred',
		useNewUrlParser: true,
		useFindAndModify: false,
		useUnifiedTopology: true
	});
	mongoose.Promise = global.Promise;
};
