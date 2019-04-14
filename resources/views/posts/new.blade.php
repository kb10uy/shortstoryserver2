@extends('layouts.default')

@section('title', '新しい作品の投稿')

@section('content')
<div class="container">
    <h1>新しい作品を投稿する</h1>

    <form name="newpost" method="POST" action="{{ route('posts.post') }}" onsubmit="return false;">
        <div class="pair">
            <label for="title">タイトル</label>
            <input type="text" name="title" id="title">
        </div>
        <div class="pair">
            <label for="body">本文</label>
            <textarea name="body" id="body" cols="30" rows="10"></textarea>
        </div>
        <div class="pair">
            <button type="button" class="button" onclick="newpost.submit();">投稿する</button>
        </div>
    </form>
</div>
@endsection
