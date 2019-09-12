<?php

namespace App\Http\Controllers;

use DateTime;
use Illuminate\Http\Request;
use Auth;
use DB;
use App\Series;
use App\Post;

class SeriesController extends Controller
{
    /**
     * GET /series/{id}.
     */
    public function show(Request $request)
    {
        /** @var Series */
        $series = Series::with('user')->findOrFail($request->id);
        $posts = $series->posts()->orderBy('pivot_order')->get();
        $isAuthor = Auth::check() && Auth::user()->id === $series->user->id;

        return view('series.show', compact('series', 'posts', 'isAuthor'));
    }

    /**
     * GET /series/latest.
     */
    public function latest(Request $request)
    {
        $series = Series::with('user')
            ->orderBy('updated_at', 'desc')
            ->paginate(10);

        return view('series.latest', compact('series'));
    }

    /**
     * POST /api/series/push.
     */
    public function push(Request $request)
    {
        $validated = $request->validate([
            'post_id' => 'required|numeric',
            'series_id' => 'required|numeric',
        ]);
        $user = Auth::user();
        $series = Series::findOrFail($validated['series_id']);
        $post = Post::findOrFail($validated['post_id']);

        if ($user->cant('addToSeries', [$post, $series])) {
            // TODO: 翻訳
            return response()
                ->json(['error' => '自分のシリーズに自分の作品のみ登録できます'], 403);
        }

        $seriesCount = DB::table('series_posts')
            ->where('series_id', $series->id)
            ->count();
        $now = new DateTime();

        DB::table('series_posts')->insertOrIgnore([
            'series_id' => $series->id,
            'post_id' => $post->id,
            'order' => $seriesCount + 1,
            'created_at' => $now,
            'updated_at' => $now,
        ]);

        return response()
            ->json(['status' => '追加しました。'], 201);
    }

    /**
     * GET /series/{id}/edit_order.
     */
    public function editOrder(Request $request)
    {
        $user = Auth::user();
        $series = Series::findOrFail($request->id);
        if ($user->cant('edit', $series)) {
            return redirect()
                ->route('index')
                ->with('error', __('statuses.not-your-seriess', ['title' => $series->title]));
        }

        $posts = $series->posts()->orderBy('pivot_order')->get();

        return view('series.edit-order', compact('series', 'posts'));
    }

    /**
     * GET /api/series/list_posts.
     */
    public function listPosts(Request $request)
    {
        $validated = $request->validate([
            'series_id' => 'required|numeric',
        ]);

        $user = Auth::user();
        $series = Series::findOrFail($validated['series_id']);
        if ($user->cant('edit', $series)) {
            return response()
                ->json(['error' => __('statuses.not-your-seriess', ['title' => $series->title])], 403);
        }

        $posts = $series
            ->posts()
            ->select(['posts.id as id', 'posts.title as title', 'posts.created_at as created_at', 'series_posts.order as original_order'])
            ->orderBy('pivot_order')
            ->get();

        return response()->json($posts);
    }

    /**
     * POST /api/series/update.
     */
    public function update(Request $request)
    {
        $validated = $request->validate([
            'series_id' => 'required|numeric',
            'data' => 'required',
        ]);
        $user = Auth::user();
        $series = Series::findOrFail($validated['series_id']);
        if ($user->cant('edit', $series)) {
            return response()
                ->json(['error' => __('statuses.not-your-seriess', ['title' => $series->title])], 403);
        }

        $reorderedItems = collect($validated['data'])->filter(function ($item) {
            return false === $item['remove'];
        })->values();

        // TODO: DELETE しなくて済む方法がありそうなんだよな
        DB::table('series_posts')->where('series_id', $series->id)->delete();
        DB::table('series_posts')->insert($reorderedItems->map(function ($item, $index) use ($series) {
            return [
                'series_id' => $series->id,
                'post_id' => $item['post_id'],
                'order' => $index + 1,
            ];
        })->toArray());

        return response()->json($reorderedItems);
    }
}
