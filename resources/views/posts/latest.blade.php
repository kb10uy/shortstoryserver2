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
            <p>
                ここに説明文が表示される予定です。実装しないかもしれません。
            </p>
            <ul class="tags">
                <li>ここに</li>
                <li>設定されているタグのリストが</li>
                <li>表示されます</li>
            </ul>
        </section>
    @endforeach

    {{ $posts->links() }}
</div>
@endsection

