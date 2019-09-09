<?php

namespace App;

use Illuminate\Database\Eloquent\Model;

class Series extends Model
{

    public function user()
    {
        return $this->belongsTo(User::class);
    }

    public function posts()
    {
        return $this->belongsToMany(Post::class, 'series_posts')->withTimestamps()->withPivot('order');
    }
}
