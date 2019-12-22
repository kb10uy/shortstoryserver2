<?php

namespace App;

use Illuminate\Database\Eloquent\Model;

/**
 * @property int    $id
 * @property string $name
 */
class Tag extends Model
{
    protected $fillable = ['name'];

    protected $visible = ['id', 'name'];

    public function posts()
    {
        return $this->belongsToMany(Post::class, 'posts_tags')
            ->withTimestamps();
    }
}
