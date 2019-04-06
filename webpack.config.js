const webpack = require('webpack');
const path = require('path');
const AutoPrefixer = require('autoprefixer');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const CleanWebpackPlugin = require('clean-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const LaravelMixManifest = require('webpack-laravel-mix-manifest');


module.exports = {
  entry: {
    app: ['./resources/scripts/app.ts', './resources/styles/app.scss'],
  },

  output: {
    path: path.resolve(__dirname, 'public'),
    filename: 'scripts/[name].[hash].js',
  },

  resolve: {
    extensions: ['.ts', '.tsx', '.js', '.scss'],
  },

  module: {
    rules: [
      {
        test: /\.tsx?$/,
        loader: 'ts-loader',
        exclude: /node_modules|vendor/,
      },
      {
        test: /\.scss$/,
        use: [
          MiniCssExtractPlugin.loader,
          {
            loader: 'css-loader',
            options: {
              importLoaders: 2,
              url: false,
            },
          },
          {
            loader: 'postcss-loader',
            options: {
              plugins: [AutoPrefixer()],
            },
          },
          {
            loader: 'sass-loader',
            options: {
              outputStyle: 'expanded',
            },
          },
        ],
        exclude: /node_modules|vendor/,
      },
    ],
  },

  optimization: {
    splitChunks: {
      cacheGroups: {
        vendor: {
          test: /node_modules/,
          name: 'vendor',
          chunks: 'initial',
          enforce: true,
        },
      },
    },
  },

  plugins: [
    new CleanWebpackPlugin({
      cleanOnceBeforeBuildPatterns: ['styles/**/*', 'scripts/**/*'],
    }),
    new MiniCssExtractPlugin({
      filename: 'styles/[name].[hash].css',
    }),
    new LaravelMixManifest(),
  ],
};
