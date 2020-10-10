const graphqlHTTP = require('express-graphql');
const { makeExecutableSchema } = require('graphql-tools');

const typeDefs = require('./typeDefs');
const resolvers = require('./resolvers');

module.exports = graphqlHTTP({
	schema: makeExecutableSchema({ typeDefs, resolvers }),
	// rootValue: root,
	graphiql: process.env.PROFILE !== 'prod' || process.env.GRAPHIQL,
});
