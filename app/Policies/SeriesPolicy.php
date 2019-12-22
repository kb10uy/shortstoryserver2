<?php

namespace App\Policies;

use App\Series;
use App\User;
use Illuminate\Auth\Access\HandlesAuthorization;

class SeriesPolicy
{
    use HandlesAuthorization;

    /**
     * Create a new policy instance.
     */
    public function __construct()
    {
    }

    public function edit(User $user, Series $series): bool
    {
        return $user->id === $series->user_id;
    }
}
