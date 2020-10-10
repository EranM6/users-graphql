const mongoose = require('mongoose');
const { userSchema } = require('./schemas');

const Users = mongoose.model('user', userSchema, 'users');

const getUser = (condition, callback) => Users.findOne(condition, callback);

const getUserById = (id, callback) => Users.findById(id, callback);

module.exports = {
	getUser,
	getUserById
}
