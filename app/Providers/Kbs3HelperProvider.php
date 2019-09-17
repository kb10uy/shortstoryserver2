<?php

namespace App\Providers;

use Illuminate\Support\ServiceProvider;

class Kbs3HelperProvider extends ServiceProvider
{
    /**
     * Register services.
     */
    public function register()
    {
        // https://s8a.jp/laravel-custom-helper
        foreach (glob(app_path() . '/Helpers/*.php') as $filename) {
            require_once $filename;
        }
    }

    /**
     * Bootstrap services.
     */
    public function boot()
    {
    }
}
