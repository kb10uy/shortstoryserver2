<?php

namespace App;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Builder;

class Post extends Model
{
    protected $fillable = [
        'title', 'body', 'body_type', 'user_id', 'description', 'visibility',
    ];

    protected $publicVisibilities = ['public', 'unlisted'];

    public function user()
    {
        return $this->belongsTo(User::class);
    }

    public function tags()
    {
        return $this->belongsToMany(Tag::class, 'posts_tags')
            ->withTimestamps();
    }

    public function scopePublic(Builder $query)
    {
        // TODO: アンチパターンの波動を感じる
        return $query->where('visibility', 'public');
    }

    public function scopeAccessible(Builder $query)
    {
        // TODO: アンチパターンの波動を感じる
        return $query->whereIn('visibility', $this->publicVisibilities);
    }

    public function isAccessible(): bool
    {
        return in_array($this->visibility, $this->publicVisibilities);
    }
}
