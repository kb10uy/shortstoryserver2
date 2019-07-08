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

Route::prefix('/help')->group(function () {
    Route::get('/about', 'HelpController@about')->name('help.about');
    Route::get('/playground', 'HelpController@playground')->name('help.playground');
    Route::get('/terms', 'HelpController@terms')->name('help.terms');
});

Route::get('/dashboard', 'DashboardController@dashboard')->name('dashboard.index');

Route::prefix('/posts')->group(function () {
    Route::get('/new', 'PostsCreationController@new')->name('posts.new');
    Route::post('/new', 'PostsCreationController@post')->name('posts.post');
});
