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

        return view('series.show', compact('series', 'posts'));
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
}
