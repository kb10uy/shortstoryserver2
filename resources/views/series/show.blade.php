@extends('layouts.default')

@section('title', htmlspecialchars($series->title))

@section('content')
<div class="container" id="app">
    <h1>{{ $series->title }} <small>by {{ $series->user->name }}</small></h1>

    <p>{{ $series->description }}</p>

    @foreach($posts as $post)
        @component('components.post-block', ['post' => $post])
        @endcomponent
    @endforeach
</div>
@endsection
