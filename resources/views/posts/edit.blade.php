@extends('layouts.default')

@section('title', __('titles.posts-edit'))

@section('includes')
<script defer src="{{ mix('/scripts/edit-post.js') }}"></script>
<link rel="stylesheet" href="{{ mix('/styles/edit-post.css') }}" media="all">
@endsection

@section('content')
<div class="container" id="app">
    <h1>@lang('titles.posts-edit')</h1>

    <form name="editpost" method="POST" action="{{ route('posts.update', ['id' => $id]) }}" onsubmit="return false;">
        @csrf
        <input type="hidden" name="_method" value="PATCH">
        <div class="pair">
            <label for="title">@lang('labels.title')</label>
            <input type="text" name="title" id="title" value="{{ $title }}">
        </div>
        <div class="pair">
            <label for="tags">@lang('labels.tags')</label>
            <tag-editor initial-tags="{{ $tags_json }}" placeholder="{{ __('labels.tags-placeholder') }}"></tag-editor>
        </div>
        <div class="pair">
            <label for="body_type">@lang('labels.body-format')</label>
            <select id="body_type" name="body_type" value="{{ $body_type }}">
                <option value="s3wf2" selected>ShortStoryServer Writer's Format v2</option>
            </select>
        </div>
        <div class="pair">
            <label for="body">@lang('labels.body-text')</label>
            <textarea name="body" id="body" cols="30" rows="10">{{ $body }}</textarea>
        </div>
        <div class="pair">
            <button type="button" class="button" onclick="editpost.submit();">@lang('actions.posts-update')</button>
        </div>
    </form>
</div>
@endsection

