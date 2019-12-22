<?php

namespace App\Http\Controllers;

use App\Post;
use Auth;
use Illuminate\Http\Request;
use App\Text\Parser\S3wf2Native;

class PostsController extends Controller
{
    /** @var S3wf2Native */
    private S3wf2Native $s3wf2;

    public function __construct(S3wf2Native $s3wf2)
    {
        $this->s3wf2 = $s3wf2;
    }

    public function latest()
    {
        $posts = Post::with(['user', 'tags'])
            ->public()
            ->orderBy('updated_at', 'desc')
            ->paginate(10);

        return view('posts.latest', compact('posts'));
    }

    public function show(Request $request)
    {
        $post = Post::with(['user', 'tags'])->findOrFail($request->id);
        $this->authorize('view', $post);

        $author = $post->user;
        $isAuthor = Auth::check() && Auth::user()->id === $author->id;

        $articleHtml = '';
        switch ($post->body_type) {
            case 's3wf2':
                $articleHtml = $this->s3wf2->generateHtml($post->body);
                break;
            default:
                return response()->view('index', [], 500);
        }

        $id = $post->id;
        $title = $post->title;
        $description = $post->description;
        $tags = $post->tags;

        return view('posts.show', compact('id', 'articleHtml', 'title', 'description', 'tags', 'author', 'isAuthor'));
    }
}
