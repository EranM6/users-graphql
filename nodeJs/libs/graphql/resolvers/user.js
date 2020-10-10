module.exports = {
	User: {
		isPayingUser: ({ products }) => products && products.length > 0
	}
}
