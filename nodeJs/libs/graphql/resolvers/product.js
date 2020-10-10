module.exports = {
	Product: {
		status: ({ statusCode }) => {
			switch (statusCode) {
				case 9:
					return 'STOPPED';
				case 99:
					return 'FROZEN';
				case 3:
					return 'ACTIVE';
				default:
					return null;
			}
		}
	}
}
