@extends('layouts.default')

@section('title', htmlspecialchars($title))

@section('includes')
<script defer src="{{ mix('/scripts/show-post.js') }}"></script>
<link rel="stylesheet" href="{{ mix('/styles/show-post.css') }}" media="all">
@endsection

@section('content')
<div class="container">
    <div class="post-info" id="app">
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
                    <a href="{{ route('posts.edit', ['id' => $id]) }}">@lang('actions.posts-edit')</a><br>
                    <a @click="showSeriesDialog" href="#">@lang('actions.posts-edit')</a>
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
                    <li class="tag"><a href="{{ route('search.tag', ['q' => $tag->name]) }}">{{ $tag->name }}</a></li>
                @endforeach
            </ul>
        @endif

        <modal-dialog button-type="ok-cancel" v-if="shown.series" @dialog-ok="addToSeries" @dialog-closed="shown.series = false">
            <template v-slot:label>シリーズに追加</template>
        </modal-dialog>
    </div>
    <hr>

    {!! $articleHtml !!}
</div>
@endsection
