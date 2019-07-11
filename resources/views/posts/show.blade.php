@extends('layouts.default')

@section('title', $title)

@section('content')
<div class="container">
    <div class="post-info">
        <h1>{{ $title }}</h1>
        <div class="user">
            <img src="https://via.placeholder.com/64" alt="{{ $author->name }}">
            <div>
                Author:<br>
                <a href="{{ route('users.show', ['name' => $author->name]) }}">{{ $author->name }} <small>&#64;{{ $author->name }}</small></a>
            </div>
            @if($isAuthor)
                <details class="author">
                    <summary>@lang('labels.author-menu')</summary>
                    <a href="{{ route('posts.edit', ['id' => $id]) }}">@lang('actions.posts-edit')</a>
                </details>
            @else
                <details>
                    <summary>@lang('labels.menu')</summary>
                </details>
            @endif
        </div>
        <p>
            あまりにもさみしいのでここになんか説明文が書けるようになるかもしれない。ならないかもしれない。
        </p>

        <ul class="tags">
            <li>タグは</li>
            <li>このように</li>
            <li>表示されます</li>
        </ul>
    </div>
    <hr>

    {!! $articleHtml !!}
</div>
@endsection
