@extends('layouts.noindex')

@section('title', __('titles.search-tag', ['name' => e($query)]))

@section('content')
<div class="container">
    <h1>@lang('titles.search-tag', ['name' => e($query)])</h1>

    @foreach($posts as $post)
        @component('components.post-block', ['post' => $post])
        @endcomponent
    @endforeach

    {{ $posts->links() }}
</div>
@endsection

