require('graphql-import-node/register');
const { mergeTypeDefs } = require('@graphql-tools/merge');

const productType = require('./product.graphql');
const queryType = require('./query.graphql');
const scalarsType = require('./scalars.graphql');
const userType = require('./user.graphql');

module.exports = mergeTypeDefs([
	productType,
	queryType,
	scalarsType,
	userType
]);
