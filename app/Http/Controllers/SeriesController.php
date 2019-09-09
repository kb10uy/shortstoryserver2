<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use App\Series;

class SeriesController extends Controller
{
    public function show(Request $request)
    {
        $series = Series::with('user')->findOrFail($request->id);
        $posts = $series->posts()->orderBy('pivot_order')->get();

        return view('series.show', compact('series', 'posts'));
    }
}
