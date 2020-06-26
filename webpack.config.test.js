const path = require('path')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

const dist = path.resolve(__dirname, 'distTest')

module.exports = {
	mode: 'development',
	target: 'node',
	entry: {
		index: './js/test_index.ts'
	},
	output: {
		path: dist,
		filename: '[name].js'
	},
	resolve: {
		extensions: ['.ts', '.js'],
	},
	module: {
		rules: [
			{
				test: /\.ts$/,
				loader: 'ts-loader',
				exclude: /node_modules/,
				options: {onlyCompileBundledFiles: true}
			},
		]
	},
	plugins: [
		new WasmPackPlugin({
			crateDirectory: __dirname,
		}),
	],
}
