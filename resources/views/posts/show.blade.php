@extends('layouts.default')

@section('title', $title)

@section('content')
<div class="container">
    <div class="post-info">
        <h1>{{ $title }}</h1>
        <div class="user">
            <img src="{{ $author->avatar_url }}" alt="{{ $author->name }}">
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
        <p class="summary">
            @if($description)
                {{ $description }}
            @else
                @lang('labels.no-description')
            @endif
        </p>

        @if($tags->isNotEmpty())
            <ul class="tags">
                @foreach($tags as $tag)
                    <li class="tag">{{ $tag->name }}</li>
                @endforeach
            </ul>
        @endif
    </div>
    <hr>

    {!! $articleHtml !!}
</div>
@endsection
