<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;

class PostsCreationController extends Controller
{
    public function __construct()
    {
        $this->middleware('auth');
    }

    public function new()
    {
        return view('posts.new');
    }
}
