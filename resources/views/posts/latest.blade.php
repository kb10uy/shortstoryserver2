@extends('layouts.noindex')

@section('title', __('titles.posts-latest'))

@section('content')
<div class="container">
    <h1>@lang('titles.posts-latest')</h1>

    @foreach($posts as $post)
        <section class="post-summary">
            <h2>
                <a href="{{ route('posts.show', ['id' => $post->id]) }}">{{ $post->title }}</a>
                <small>by {{ $post->user->name }}</small>
            </h2>
            <p class="summary">
                ここに説明文が表示される予定です。実装しないかもしれません。
            </p>
            @if($post->tags->isNotEmpty())
                <ul class="tags">
                    @foreach($post->tags as $tag)
                        <li class="tag">{{ $tag->name }}</li>
                    @endforeach
                </ul>
            @endif
        </section>
    @endforeach

    {{ $posts->links() }}
</div>
@endsection

