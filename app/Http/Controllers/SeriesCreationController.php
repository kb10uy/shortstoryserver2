<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;

class SeriesCreationController extends Controller
{
    public function new()
    {
        return view('series.new');
    }

    public function post()
    {

    }
}
