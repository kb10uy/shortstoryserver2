<?php

namespace App\Http\Controllers;

use Illuminate\Validation\Rule;
use Illuminate\Http\Request;
use Illuminate\Pagination\LengthAwarePaginator;
use App\Post;
use App\User;
use App\Tag;

class SearchController extends Controller
{
    /**
     * GET /search.
     */
    public function index(Request $request)
    {
        $validated = $request->validate([
            'query' => 'nullable|max:500',
            'type' => ['nullable', Rule::in(['keyword', 'tag', 'user'])],
            'sort' => ['nullable', Rule::in(['created', 'updated'])],
        ]);

        if (!isset($validated['query'])) {
            return view('search.index', [
                'keyword' => '',
                'titleKeyword' => '',
                'selectedType' => 'keyword',
                'selectedSort' => 'updated',
            ]);
        }

        switch ($validated['type'] ?? null) {
            // 全文検索
            case 'keyword':
            default:
                // TODO: タイトルも入れたほうがいいと思う
                $query = Post::with(['tags', 'user'])->whereRaw('body &@~ ?', [$validated['query']]);
                break;

            // タグ検索
            case 'tag':
                $tag = Tag::where('name', $validated['query'])->first();
                if (!$tag) {
                    $query = Post::with(['tags', 'user'])->where('body', '');
                } else {
                    $query = $tag->posts()->with(['tags', 'user']);
                }
                break;

            // ユーザー検索
            case 'user':
                $user = User::where('name', $validated['query'])->first();
                if (!$user) {
                    $query = Post::with(['tags', 'user'])->where('body', '');
                } else {
                    $query = $user->posts()->with(['tags', 'user']);
                }
                break;
        }

        switch ($validated['sort'] ?? null) {
            case 'updated':
            default:
                $query = $query->orderBy('updated_at', 'DESC');
                break;
            case 'created':
                $query = $query->orderBy('created_at', 'DESC');
                break;
        }

        $posts = $query->public()->paginate(10);

        return view('search.index', [
            'titleKeyword' => $validated['query'] ?? '',
            'keyword' => $validated['query'] ?? '',
            'selectedType' => $validated['type'],
            'selectedSort' => $validated['sort'],
            'searchResult' => $posts,
        ]);
    }
}
