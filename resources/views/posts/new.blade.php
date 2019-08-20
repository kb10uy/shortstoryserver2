@extends('layouts.default')

@section('title', __('titles.posts-new'))

@section('includes')
<script defer src="{{ mix('/scripts/edit-post.js') }}"></script>
<link rel="stylesheet" href="{{ mix('/styles/edit-post.css') }}" media="all">
@endsection

@section('content')
<div class="container" id="app">
    <h1>@lang('titles.posts-new')</h1>

    <form name="newpost" method="POST" action="{{ route('posts.post') }}" onsubmit="return false;">
        @csrf
        <div class="pair">
            <label for="title">@lang('labels.title')</label>
            <input type="text" name="title" id="title">
        </div>
        <div class="pair">
            <label for="tags">@lang('labels.tags')</label>
            <tag-editor placeholder="{{ __('labels.tags-placeholder') }}"></tag-editor>
        </div>
        <div class="pair">
            <label for="body_type">@lang('labels.body-format')</label>
            <select id="body_type" name="body_type">
                <option value="s3wf2" selected>ShortStoryServer Writer's Format v2</option>
            </select>
        </div>
        <div class="pair">
            <label for="body">@lang('labels.body-text')</label>
            <textarea name="body" id="body" cols="30" rows="10"></textarea>
        </div>
        <div class="pair">
            <button type="button" class="button" onclick="newpost.submit();">@lang('actions.posts-submit')</button>
        </div>
    </form>
</div>
@endsection
