const { GraphQLScalarType, } = require('graphql');
const { Kind, } = require('graphql/language');
const { GraphQLError, } = require('graphql/error');

const isDate = value => toString.call(value) === '[object Date]' && !isNaN(value.valueOf());

const coerceDate = value => {
	const date = new Date(value);
	if (!isDate(date)) {
		const message = `Date can't represent non-date value: ${value}`;
		throw new TypeError(message);
	}
	return date;
};

module.exports = new GraphQLScalarType({
	name: 'Date',
	serialize: value => coerceDate(value)
		.toISOString(),
	parseValue: coerceDate,
	parseLiteral(valueNode) {
		const { kind, value, } = valueNode;
		let date;
		switch (kind) {
			case Kind.INT:
			case Kind.FLOAT:
				date = new Date(+value);
				break;
			case Kind.STRING:
				date = new Date(value);
				break;
			default:
		}
		if (!isDate(date)) {
			throw new GraphQLError(`Expected date value but got: ${value}`, [
				valueNode,
			]);
		}
		return date;
	},
})
