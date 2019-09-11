@extends('layouts.noindex')

@section('title', htmlspecialchars(__('titles.series-edit-order', ['title' => $series->title])))

@section('includes')
<script defer src="{{ mix('/scripts/edit-series.js') }}"></script>
@endsection

@section('content')
<div class="container" id="app">
    <h1>{{ __('titles.series-edit-order', ['title' => $series->title]) }}</h1>
    <p>
        左端から元の順序、タイトル、作品の投稿日時です。
    </p>

    <draggable v-model="posts" class="series-editor" handle=".handle" v-cloak>
        <div v-for="post of posts" :key="post.id" class="item">
            <span class="handle"><i class="handle fas fa-grip-lines"></i></span>
            <span class="order">@{{ post.original_order }}</span>
            <span class="title">@{{ post.title }}</span>
            <span class="date">@{{ post.created_at }}</span>
        </div>
    </draggable>
</div>
@endsection

