const path = require('path');
const webpack = require('webpack');

module.exports = {
  entry: './src/front/index.js',
  output: {
    path: path.resolve(__dirname, 'src/front'),
    filename: 'bundle.js'
  },
  module: {
    rules: [
        {
          test: /\.css$/,
          exclude: /node_modules/,
          use: ['style-loader', 'css-loader']
        },
        {
          test: /\.js$/,
          exclude: /node_modules/,
          use: {
            loader: 'babel-loader',
            options: { presets: ["env", "react"] }
          }
        }
      ]
  }
}

