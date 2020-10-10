const mongoose = require('mongoose');
const Double = require('@mongoosejs/double');

const product = {
	prodNum: { type: Number },
	statusCode: { type: Number },
	status: { type: String, enum: ['ACTIVE', 'FROZEN', 'STOPPED'] },
	saleCode: { type: Number },
};

module.exports = mongoose.Schema({
	_id: { type: mongoose.ObjectId, required: true },
	firstName: { type: String },
	lastName: { type: String },
	email: { type: String, required: true, index: true, unique: true },
	phone: { type: String },
	updateDate: { type: Date, default: new Date() },
	ticket: { type: Double },
	ticketExpiration: { type: Date },
	registerDate: { type: Date },
	products: { type: [product] },
});
