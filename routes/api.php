<?php

/*
|--------------------------------------------------------------------------
| API Routes
|--------------------------------------------------------------------------
|
| Here is where you can register API routes for your application. These
| routes are loaded by the RouteServiceProvider within a group which
| is assigned the "api" middleware group. Enjoy building your API!
|
 */

Route::prefix('/users')->group(function () {
    Route::get('/latest_posts', 'UsersController@latestPosts')->name('api.users.latest-posts');

    // ユーザー情報が必要な API
    Route::middleware('auth')->group(function () {
        Route::get('/latest_user_posts', 'UsersController@latestUserPosts')->name('api.users.latest-user-posts');
        Route::get('/latest_user_series', 'UsersController@latestUserSeries')->name('api.users.latest-user-series');
    });
});

Route::prefix('/series')->group(function () {
    Route::middleware('auth')->group(function () {
        Route::get('/list_posts', 'SeriesController@listPosts')->name('api.series.list-posts');
        Route::post('/push', 'SeriesController@push')->name('api.series.push');
        Route::post('/update', 'SeriesController@update')->name('api.series.update');
    });
});
