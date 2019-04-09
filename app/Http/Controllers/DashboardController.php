<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;

class DashboardController extends Controller
{
    public function __construct()
    {
        $this->middleware('auth');
    }

    /**
     * ダッシュボード
     */
    public function dashboard()
    {
        return view('dashboard.index');
    }
}
