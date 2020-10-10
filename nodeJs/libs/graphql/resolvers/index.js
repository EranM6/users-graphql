const { mergeResolvers } = require('@graphql-tools/merge');

const productResolver = require('./product');
const queryResolver = require('./query');
const scalarsResolver = require('./scalars');
const userResolver = require('./user');

module.exports = mergeResolvers([
	productResolver,
	queryResolver,
	scalarsResolver,
	userResolver,
]);
