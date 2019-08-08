<?php

namespace App\Http\Controllers;

use Auth;
use Illuminate\Http\Request;
use App\Post;
use Lib\Formats\S3wf2\S3wf2Format;

class PostsController extends Controller
{
    public function latest()
    {
        $posts = Post::with('user')->orderBy('updated_at', 'desc')->take(10)->get();

        return view('posts.latest', compact('posts'));
    }

    public function show(Request $request)
    {
        $post = Post::findOrFail($request->id);
        $author = $post->user;
        $isAuthor = Auth::check() && Auth::user()->id === $author->id;

        switch ($post->body_type) {
            case 's3wf2':
                $format = new S3wf2Format();
                break;
            default:
                return response()->view('index', [], 500);
        }
        $format->parse($post->body);

        $id = $post->id;
        $title = $post->title;
        $articleHtml = $format->toHtml();

        return view('posts.show', compact('id', 'articleHtml', 'title', 'author', 'isAuthor'));
    }
}
