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
        </div>
        <ul class="tags">
        </ul>
    </div>
    <hr>

    {!! $articleHtml !!}
</div>
@endsection
