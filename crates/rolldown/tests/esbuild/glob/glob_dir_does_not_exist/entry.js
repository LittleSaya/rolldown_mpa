const ab = Math.random() < 0.5 ? 'a.js' : 'b.js'
console.log({
	concat: {
		require: require('./src/' + ab),
		import: import('./src/' + ab),
	},
	template: {
		require: require(`./src/${ab}`),
		import: import(`./src/${ab}`),
	},
})