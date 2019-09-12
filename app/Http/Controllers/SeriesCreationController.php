<?php

namespace App\Http\Controllers;

use Auth;
use Illuminate\Http\Request;
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

    public function edit(Request $request)
    {
        $series = Series::findOrFail($request->id);
        $user = Auth::user();
        if ($user->cant('edit', $series)) {
            return redirect()
                ->route('index')
                ->with('error', __('statuses.not-your-series'));
        }

        return view('series.edit', [
            'id' => $series->id,
            'title' => $series->title,
            'description' => $series->description,
        ]);
    }

    public function update(Request $request, UpsertSeries $input)
    {
        $series = Series::findOrFail($request->id);
        $user = Auth::user();
        if ($user->cant('edit', $series)) {
            return redirect()
                ->route('index')
                ->with('error', __('statuses.not-your-series'));
        }

        $series->title = $input->title;
        $series->description = $input->description;
        $series->save();

        return redirect()->route('series.show', ['id' => $series->id])->with('status', __('statuses.series-updated'));
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
