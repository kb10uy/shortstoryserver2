@extends('layouts.noindex')

@section('title', __('titles.dashboard-posts'))

@section('includes')
<script defer src="{{ mix('/scripts/dashboard.js') }}"></script>
<link rel="stylesheet" href="{{ mix('/styles/dashboard.css') }}" media="all">
@endsection

@section('content')
<div class="container">
    <div class="dashboard">
        <div class="left">
            @component('dashboard.sidebar', ['selected' => 'posts'])
            @endcomponent
        </div>
        <div class="right">
            <div class="box" id="app">
                <h1>@lang('titles.dashboard-posts')</h1>

                <dashboard-posts user-id="{{ Auth::user()->id }}" />
            </div>
        </div>
    </div>
</div>
@endsection
