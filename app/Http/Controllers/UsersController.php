<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use DB;
use App\Post;
use App\User;

class UsersController extends Controller
{
    public function show(Request $request, string $name)
    {
        $user = User::where('name', $name)->firstOrFail();
        $postsCount = Post::where('user_id', $user->id)->count();

        return view('users.show', compact('user', 'postsCount'));
    }

    public function latestPosts(Request $request)
    {

    }
}
