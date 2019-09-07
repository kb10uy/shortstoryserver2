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
    Route::get('/latest_user_posts', 'UsersController@latestUserPosts')->name('api.users.latest-user-posts');
});
