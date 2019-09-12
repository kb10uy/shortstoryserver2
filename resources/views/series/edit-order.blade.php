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

    <p><button class="button" @click="sendData" :disabled="sending">保存</button></p>
    <draggable v-model="posts" class="series-editor" handle=".handle" v-cloak>
        <div v-for="(post, index) of posts" :key="post.id" class="item" :class="{ inactive: post.willBeDeleted }">
            <span class="handle"><i class="handle fas fa-grip-lines"></i></span>
            <span class="order">@{{ post.original_order }}</span>
            <span class="title">
                <button class="mini button" :class="{ warning: !post.willBeDeleted }" @click="toggleDeletion(index)">@{{ post.willBeDeleted ? '保持する' : '削除する' }}</button>
                @{{ post.title }}
            </span>
            <span class="date">@{{ post.created_at }}</span>
        </div>
    </draggable>
</div>
@endsection

