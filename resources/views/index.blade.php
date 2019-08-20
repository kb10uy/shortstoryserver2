@extends('layouts.default')

@section('title', 'ホーム')

@section('content')
<div class="container">
    <div class="center">
        <img id="home-logo" src="/images/newlogo-full.png" alt="ShortStoryServer">
        <h1>@lang('titles.index-motto')</h1>
        <p>おかげさまで ShortStoryServer は第 3 世代を迎えました。</p>
    </div>
</div>
@endsection
