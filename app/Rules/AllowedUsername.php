<?php

namespace App\Rules;

use Illuminate\Contracts\Validation\Rule;

class AllowedUsername implements Rule
{
    /**
     * キーワード的に使えないユーザー名.
     */
    protected const PROHIBITED_NAMES = [
        'root',
        'user', 'users',
        'setting', 'settings',
        'post', 'posts',
        'bookmark', 'bookmarks',
        'series',
    ];

    public function __construct()
    {
    }

    /**
     * Determine if the validation rule passes.
     *
     * @param string $attribute
     * @param mixed  $value
     *
     * @return bool
     */
    public function passes($attribute, $value)
    {
        if (!is_string($value)) {
            return false;
        }

        return preg_match('/^[a-z0-9_]{2,64}$/iu', $value)
            && in_array($value, self::PROHIBITED_NAMES);
    }

    public function message()
    {
        return ':attribute is invalid as an username.';
    }
}
