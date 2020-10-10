const { GraphQLScalarType, } = require('graphql');
const { Kind, } = require('graphql/language');

MAX_LONG = Number.MAX_SAFE_INTEGER;
MIN_LONG = Number.MIN_SAFE_INTEGER;

const coerceLong = function(value) {
	let num;
	if (value === '') {
		throw new TypeError('Long cannot represent non 52-bit signed integer value: (empty string)');
	}
	num = Number(value);
	if (num === num && num <= MAX_LONG && num >= MIN_LONG) {
		if (num < 0) {
			return Math.ceil(num);
		} else {
			return Math.floor(num);
		}
	}
	throw new TypeError('Long cannot represent non 52-bit signed integer value: ' + String(value));
};

module.exports = new GraphQLScalarType({
	name: 'Long',
	serialize: coerceLong,
	parseValue: coerceLong,
	parseLiteral(ast) {
		let num;
		if (ast.kind === Kind.INT) {
			num = parseInt(ast.value, 10);
			if (num <= MAX_LONG && num >= MIN_LONG) {
				return num;
			}
			return null;
		}
	},
})
