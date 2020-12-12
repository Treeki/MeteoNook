const child_process = require('child_process')
const path = require('path')
const CopyPlugin = require('copy-webpack-plugin')
const TerserPlugin = require('terser-webpack-plugin')
const OptimizeCSSAssetsPlugin = require('optimize-css-assets-webpack-plugin')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const MiniCssExtractPlugin = require('mini-css-extract-plugin')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const { CleanWebpackPlugin } = require('clean-webpack-plugin')
const { VueLoaderPlugin } = require('vue-loader')
const { DefinePlugin } = require('webpack')

const gitCommitShort = child_process.execSync('git rev-parse --short HEAD').toString().trim()
const gitCommitHash = child_process.execSync('git rev-parse HEAD').toString().trim()
const gitURL = 'https://github.com/Treeki/MeteoNook/commit/' + gitCommitHash
const gitCommitStamp = parseInt(child_process.execSync('git log -1 --date=unix --format=%cd').toString().trim(), 10)

const dist = path.resolve(__dirname, 'dist')

module.exports = {
	mode: 'production',
	entry: {
		index: './js/app_index.ts'
	},
	output: {
		path: dist,
		filename: '[name].[contenthash].js',
		chunkFilename: '[name].[contenthash].js'
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
				test: /\.css$/i,
				use: [
					//{loader: 'micro-style-loader'},
					{loader: MiniCssExtractPlugin.loader, options: {esModule: true}},
					{loader: 'css-loader', options: {modules: false, esModule: true}},
				]
			},
			{
				test: /\.s[ac]ss$/i,
				use: [
					//{loader: 'micro-style-loader'},
					{loader: MiniCssExtractPlugin.loader, options: {esModule: true}},
					{loader: 'css-loader', options: {modules: false, esModule: true}},
					'sass-loader',
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
		new CleanWebpackPlugin(),

		new HtmlWebpackPlugin({
			hash: true,
			template: 'index.html',
			favicon: 'static/favicon.ico',
		}),

		new CopyPlugin({
			patterns: [path.resolve(__dirname, 'static')]
		}),

		new WasmPackPlugin({
			crateDirectory: __dirname,
		}),

		new VueLoaderPlugin(),
		new MiniCssExtractPlugin({
			filename: '[name].[contenthash].css',
			chunkFilename: '[name].[contenthash].css'
		}),

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
