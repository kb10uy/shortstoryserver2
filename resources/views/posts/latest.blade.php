@extends('layouts.noindex')

@section('title', __('titles.posts-latest'))

@section('content')
<div class="container">
    <h1>@lang('titles.posts-latest')</h1>

    @foreach($posts as $post)
        @component('components.post-block', ['post' => $post])
        @endcomponent
    @endforeach

    {{ $posts->links() }}
</div>
@endsection

