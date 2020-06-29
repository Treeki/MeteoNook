const child_process = require('child_process')
const path = require('path')
const CopyPlugin = require('copy-webpack-plugin')
const TerserPlugin = require('terser-webpack-plugin')
const OptimizeCSSAssetsPlugin = require('optimize-css-assets-webpack-plugin')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const MiniCssExtractPlugin = require('mini-css-extract-plugin')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const { VueLoaderPlugin } = require('vue-loader')
const { DefinePlugin } = require('webpack')

const gitCommitShort = child_process.execSync('git rev-parse --short HEAD').toString().trim()
const gitCommitHash = child_process.execSync('git rev-parse HEAD').toString().trim()
const gitURL = 'https://github.com/Treeki/MeteoNook/commit/' + gitCommitHash
const gitCommitStamp = child_process.execSync('git log -1 --format=%cd').toString().trim()

const dist = path.resolve(__dirname, 'dist')

module.exports = {
	mode: 'production',
	entry: {
		index: './js/app_index.ts'
	},
	output: {
		path: dist,
		filename: '[name].js'
	},
	devServer: {
		contentBase: dist,
	},
	resolve: {
		extensions: ['.ts', '.js'],
	},
	module: {
		rules: [
			{
				test: /\.css$/,
				use: [
					//{loader: 'micro-style-loader'},
					{loader: MiniCssExtractPlugin.loader, options: {esModule: true}},
					{loader: 'css-loader', options: {modules: false, esModule: true}}
				]
			},
			{
				test: /\.vue$/,
				loader: 'vue-loader'
			},
			{
				test: /\.ts?$/,
				loader: 'ts-loader',
				exclude: /node_modules/,
				options: {appendTsSuffixTo: [/\.vue$/]}
			},
		]
	},
	plugins: [
		new HtmlWebpackPlugin({
			hash: true,
			title: 'MeteoNook Alpha'
		}),

		new CopyPlugin({
			patterns: [path.resolve(__dirname, 'static')]
		}),

		new WasmPackPlugin({
			crateDirectory: __dirname,
		}),

		new VueLoaderPlugin(),
		new MiniCssExtractPlugin(),

		new DefinePlugin({
			METEONOOK_GIT_COMMIT_SHORT: JSON.stringify(gitCommitShort),
			METEONOOK_GIT_COMMIT: JSON.stringify(gitCommitHash),
			METEONOOK_GIT_COMMIT_URL: JSON.stringify(gitURL),
			METEONOOK_GIT_COMMIT_STAMP: JSON.stringify(gitCommitStamp),
		})
	],
	optimization: {
		minimize: true,
		concatenateModules: false,
		minimizer: [new TerserPlugin(), new OptimizeCSSAssetsPlugin()],
	},
}
