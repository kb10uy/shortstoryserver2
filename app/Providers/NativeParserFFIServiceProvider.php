<?php

namespace App\Providers;

use Illuminate\Support\ServiceProvider;
use App\Text\Parser\S3wf2Native;

class NativeParserFFIServiceProvider extends ServiceProvider
{
    public function provides()
    {
        return [S3wf2Native::class];
    }

    /**
     * Register services.
     *
     * @return void
     */
    public function register()
    {
        $this->app->singleton(S3wf2Native::class, function ($app) {
            // TODO: config/*.php から取るようにしたほうがいいかも
            $basePath = base_path();
            $relativePath = env('LIBS3WF2_PATH', '/usr/local/lib/libs3wf2.so');
            $libraryPath = realpath($basePath . '/' . $relativePath);

            return new S3wf2Native($libraryPath);
        });
    }

    /**
     * Bootstrap services.
     *
     * @return void
     */
    public function boot()
    {
    }
}
