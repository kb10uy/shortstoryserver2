<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Pagination\LengthAwarePaginator;
use App\Tag;

class SearchController extends Controller
{
    /**
     * タグ検索
     */
    public function tag(Request $request)
    {
        $validated = $request->validate([
            'q' => 'required|max:128',
        ]);

        $query = $validated['q'];
        $tag = Tag::where('name', $query)->first();
        if (!$tag) {
            $posts = new LengthAwarePaginator(collect(), 0, 10, 1);
        } else {
            $posts = $tag
                ->posts()->with(['user', 'tags'])
                ->public()
                ->orderBy('updated_at', 'desc')
                ->paginate(10);
        }

        return view('search.tag', compact('query', 'posts'));
    }
}
