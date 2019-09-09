<?php

namespace App;

use Illuminate\Database\Eloquent\Model;

class Series extends Model
{
    public function posts()
    {
        return $this->belongsToMany(Post::class, 'series_posts')->withTimestamps()->withPivot('order');
    }
}
