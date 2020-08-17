const webpack = require('webpack');
const path = require('path');
const AutoPrefixer = require('autoprefixer');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const LaravelMixManifest = require('webpack-laravel-mix-manifest');
const VueLoaderPlugin = require('vue-loader/lib/plugin');


module.exports = {
  entry: {
    app: ['./resources/scripts/app.ts', './resources/styles/app.scss'],
    'dashboard': './resources/scripts/pages/dashboard.ts',
    'playground': './resources/scripts/pages/playground.ts',
    'edit-series': './resources/scripts/pages/edit-series.ts',
    'edit-post': './resources/scripts/pages/edit-post.ts',
    'show-user': './resources/scripts/pages/show-user.ts',
    'show-post': './resources/scripts/pages/show-post.ts',
  },

  output: {
    path: path.resolve(__dirname, 'public'),
    publicPath: '/',
    filename: 'scripts/[name].[hash].js',
  },

  resolve: {
    extensions: ['.ts', '.tsx', '.js', '.scss', '.wasm'],
    alias: {
      'vue$': 'vue/dist/vue.esm.js',
    },
  },

  module: {
    rules: [
      {
        test: /\.tsx?$/,
        loader: 'ts-loader',
        exclude: /node_modules|vendor/,
        options: {
          appendTsSuffixTo: [/\.vue$/],
        },
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
              sassOptions: {
                outputStyle: 'expanded',
              },
            },
          },
        ],
        exclude: /node_modules|vendor/,
      },
      {
        test: /\.vue$/,
        loader: 'vue-loader',
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
      cleanAfterEveryBuildPatterns: ['!images/**/*'],
    }),
    new CopyWebpackPlugin({
      patterns: [
        {
          from: './resources/assets/images',
          to: './images'
        },
        {
          from: './resources/assets/favicon.ico',
          to: './favicon.ico',
        },
      ],
    }),
    new MiniCssExtractPlugin({
      filename: 'styles/[name].[hash].css',
    }),
    new VueLoaderPlugin(),
    new LaravelMixManifest(),
  ],
};
