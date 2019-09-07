<?php

namespace App\Policies;

use App\User;
use App\Post;
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

    public function view(?User $user, Post $post)
    {
        if ($post->isAccessible()) {
            return true;
        }

        return $user && $post->user_id === $user->id;
    }
}
