<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;

class HelpController extends Controller
{
    public function about()
    {
        return view('help.about');
    }

    public function terms()
    {
        return view('');
    }

    public function playground()
    {
        return view('');
    }
}
