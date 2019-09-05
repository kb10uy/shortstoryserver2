@extends('layouts.noindex')

@section('title', __('titles.error-404'))

@section('content')
<div class="container">
    <h1>@lang('titles.error-404')</h1>
    <p>
        おさがしの黒歴史はみつかりませんでした。
    </p>
</div>
@endsection
