@extends('layouts.default')

@section('title', htmlspecialchars($series->title))

@section('content')
<div class="container" id="app">
    <h1>{{ $series->title }} <small>by {{ $series->user->name }}</small></h1>

    <p>{{ $series->description }}</p>

    @if($isAuthor)
    <p>
        <a href="{{ route('series.edit', ['id' => $series->id]) }}">@lang('actions.series-edit')</a><br>
        <a href="{{ route('series.edit-order', ['id' => $series->id]) }}">@lang('actions.series-edit-order')</a>
    </p>
    @endif

    @foreach($posts as $post)
        @component('components.post-block', ['post' => $post])
        @endcomponent
    @endforeach
</div>
@endsection
