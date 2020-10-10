const { getUser, getUserById } = require('../../mongo/model');

module.exports = {
	Query: {
		getUserById: (_, { id }) => {
			return getUserById(id)
		},
		getUserByMail: (_, { mail }) => {
			return getUser({ email: mail })
		}
	}
}
