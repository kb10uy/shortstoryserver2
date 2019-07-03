<?php

namespace App;

use Illuminate\Database\Eloquent\Model;

class Post extends Model
{
    protected $fillable = [
        'title', 'body', 'body_type',
    ];

    /**
     * body_type パラメータによって body から HTML を生成する。
     * HTML として安全な文字列を返さなければならない。
     */
    public function getHtmlBodyAttribute()
    {
    }
}
