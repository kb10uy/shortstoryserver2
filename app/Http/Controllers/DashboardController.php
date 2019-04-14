<?php

namespace App\Http\Controllers;

class DashboardController extends Controller
{
    public function __construct()
    {
        $this->middleware('auth');
    }

    /**
     * ダッシュボード.
     */
    public function dashboard()
    {
        return view('dashboard.index');
    }
}
