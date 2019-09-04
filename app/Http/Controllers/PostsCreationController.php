<?php

namespace App\Http\Controllers;

use Auth;
use Illuminate\Http\Request;
use App\Post;
use App\Tag;
use App\Http\Requests\UpsertPost;

class PostsCreationController extends Controller
{
    public function __construct()
    {
        $this->middleware('auth');
    }

    /**
     * GET /posts/new.
     */
    public function new()
    {
        return view('posts.new');
    }

    /**
     * GET /posts/:id/edit.
     */
    public function edit(Request $request)
    {
        $target = Post::with('tags')->findOrFail($request->id);
        $user = Auth::user();
        if ($user->cant('edit', $target)) {
            return redirect()
                ->route('index')
                ->with('error', __('statuses.not-your-post'));
        }

        $tagsJson = json_encode($target->tags->map(function ($tag) { return $tag->name; })->toArray());

        return view('posts.edit', [
            'id' => $target->id,
            'title' => $target->title,
            'body_type' => $target->body_type,
            'tags_json' => $tagsJson,
            'body' => $target->body,
            'description' => $target->description,
        ]);
    }

    public function post(UpsertPost $input)
    {
        $post = Post::create([
            'title' => $input->title,
            'body_type' => $input->body_type,
            'body' => $input->body,
            'user_id' => Auth::id(),
            'description' => $input->description,
        ]);
        $tags = json_decode($input->tags_json);
        $post->tags()->sync($this->tagsToIds($tags));

        return redirect()
            ->route('posts.show', ['id' => $post->id])
            ->with('status', __('statuses.post-created'));
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
            'description' => $input->description,
        ])->save();
        $tags = json_decode($input->tags_json);
        $target->tags()->sync($this->tagsToIds($tags));

        return redirect()
            ->route('posts.show', ['id' => $target->id])
            ->with('status', __('statuses.post-updated'));
    }

    /**
     * タグを全て存在する状態にしてそれらの ID を返す.
     */
    private function tagsToIds(array $tags): array
    {
        $tagIds = [];
        foreach ($tags as $tagName) {
            if (!is_string($tagName)) {
                continue;
            }
            $tag = Tag::firstOrCreate(['name' => $tagName]);
            $tagIds[] = $tag->id;
        }

        return $tagIds;
    }
}
