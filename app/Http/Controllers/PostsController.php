<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use App\Post;
use Lib\Formats\S3wf2\S3wf2Format;

class PostsController extends Controller
{
    public function show(Request $request)
    {
        $post = Post::findOrFail($request->id);
        $author = $post->user;

        switch ($post->body_type) {
            case 's3wf2':
                $format = new S3wf2Format;
                break;
            default:
                return response()->view('index', [], 500);
        }

        $format->parse($post->body);
        $title = $post->title;
        $articleHtml = $format->toHtml();

        return view('posts.show', compact('articleHtml', 'title'));
    }
}
