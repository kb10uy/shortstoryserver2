<?php

namespace App\Http\Controllers;

use App\Post;
use App\Series;
use App\User;
use Auth;
use Illuminate\Http\Request;

class UsersController extends Controller
{
    public function show(Request $request, string $name)
    {
        $user = User::where('name', $name)->firstOrFail();
        $postsCount = Post::where('user_id', $user->id)->count();

        return view('users.show', compact('user', 'postsCount'));
    }

    public function latestPosts(Request $request)
    {
        $validated = $request->validate([
            'user_id' => 'required|numeric',
            'max_id' => 'nullable|numeric',
        ]);

        $postsQuery = Post::select(['id', 'title', 'description'])
            ->with('tags')
            ->public()
            ->where('user_id', $validated['user_id']);
        if (isset($validated['max_id'])) {
            $postsQuery = $postsQuery->where('id', '<', $validated['max_id']);
        }

        $posts = $postsQuery
            ->orderBy('updated_at', 'desc')
            ->limit(10)
            ->get();

        return response()->json($posts);
    }

    public function latestUserPosts(Request $request)
    {
        $validated = $request->validate([
            'max_id' => 'nullable|numeric',
        ]);

        $postsQuery = Post::select(['id', 'title', 'description', 'visibility'])
            ->with('tags')
            ->where('user_id', Auth::user()->id);

        if (isset($validated['max_id'])) {
            $postsQuery = $postsQuery->where('id', '<', $validated['max_id']);
        }

        $posts = $postsQuery
            ->orderBy('updated_at', 'desc')
            ->limit(10)
            ->get();

        return response()->json($posts);
    }

    public function latestUserSeries(Request $request)
    {
        $validated = $request->validate([
            'max_id' => 'nullable|numeric',
        ]);

        $seriesQuery = Series::select(['id', 'title', 'description'])
            ->where('user_id', Auth::user()->id);

        if (isset($validated['max_id'])) {
            $seriesQuery = $seriesQuery->where('id', '<', $validated['max_id']);
        }

        $series = $seriesQuery
            ->orderBy('updated_at', 'desc')
            ->get();

        return response()->json($series);
    }
}
