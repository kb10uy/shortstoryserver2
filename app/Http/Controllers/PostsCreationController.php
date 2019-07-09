<?php

namespace App\Http\Controllers;

use Auth;
use Illuminate\Http\Request;
use Illuminate\Validation\Rule;
use App\Post;

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

    public function post(Request $request)
    {
        $validated = $request->validate([
            'title' => 'required|max:128',
            'body_type' => ['required', Rule::in(['s3wf2'])],
            'body' => 'required|max:1024000',
        ]);

        $post = Post::create([
            'title' => $validated['title'],
            'body_type' => $validated['body_type'],
            'body' => $validated['body'],
            'user_id' => Auth::id(),
        ]);

        return redirect()->route('posts.show', ['id' => $post->id]);
    }
}
