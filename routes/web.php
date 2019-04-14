<?php

/*
|--------------------------------------------------------------------------
| Web Routes
|--------------------------------------------------------------------------
|
| Here is where you can register web routes for your application. These
| routes are loaded by the RouteServiceProvider within a group which
| contains the "web" middleware group. Now create something great!
|
*/

Auth::routes();

Route::get('/', function () {
    return view('index');
})->name('index');

Route::get('/dashboard', 'DashboardController@dashboard')->name('dashboard.index');

Route::prefix('/posts')->group(function () {
    Route::get('/new', 'PostsCreationController@new')->name('posts.new');
    Route::post('/new', 'PostsCreationController@post')->name('posts.post');
});
