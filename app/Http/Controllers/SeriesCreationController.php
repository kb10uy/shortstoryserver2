<?php

namespace App\Http\Controllers;

use Auth;
use App\Series;
use App\Http\Requests\UpsertSeries;

class SeriesCreationController extends Controller
{
    public function __construct()
    {
        $this->middleware('auth');
    }

    public function new()
    {
        return view('series.new');
    }

    public function post(UpsertSeries $request)
    {
        $series = new Series();
        $series->title = $request->title;
        $series->description = $request->description;
        $series->user_id = Auth::user()->id;
        $series->save();

        return redirect()->route('series.show', ['id' => $series->id])->with('status', __('statuses.series-created'));
    }
}
