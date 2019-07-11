<?php

namespace App\Http\Controllers;

use Auth;
use Illuminate\Http\Request;
use Illuminate\Validation\Rule;
use App\Post;
use App\Http\Requests\UpsertPost;

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

    public function post(UpsertPost $input)
    {
        $post = Post::create([
            'title' => $input->title,
            'body_type' => $input->body_type,
            'body' => $input->body,
            'user_id' => Auth::id(),
        ]);

        return redirect()
            ->route('posts.show', ['id' => $post->id])
            ->with('status', __('statuses.post-created'));
    }

    public function edit(Request $request)
    {
        $target = Post::findOrFail($request->id);
        $user = Auth::user();
        if ($user->cant('edit', $target)) {
            return redirect()
                ->route('index')
                ->with('error', __('statuses.not-your-post'));
        }

        return view('posts.edit', [
            'id' => $target->id,
            'title' => $target->title,
            'body_type' => $target->body_type,
            'body' => $target->body,
        ]);
    }

    public function update(Request $request, UpsertPost $input)
    {
        $target = Post::findOrFail($request->id);
        $user = Auth::user();
        if ($user->cant('edit', $target)) {
            return redirect()
                ->route('index')
                ->with('error', __('statuses.not-your-post'));
        }

        $target->fill([
            'title' => $input->title,
            'body_type' => $input->body_type,
            'body' => $input->body,
        ])->save();

        return redirect()
            ->route('posts.show', ['id' => $target->id])
            ->with('status', __('statuses.post-updated'));
    }
}
