<?php

namespace App\Policies;

use App\Post;
use App\Series;
use App\User;
use Illuminate\Auth\Access\HandlesAuthorization;

class PostPolicy
{
    use HandlesAuthorization;

    /**
     * Create a new policy instance.
     */
    public function __construct()
    {
    }

    public function edit(User $user, Post $post)
    {
        return $user->id === $post->user_id;
    }

    public function addToSeries(User $user, Post $post, Series $series)
    {
        return $user->id === $post->user_id && $user->id === $series->user_id;
    }

    public function view(?User $user, Post $post)
    {
        if ($post->isAccessible()) {
            return true;
        }

        return $user && $post->user_id === $user->id;
    }
}
