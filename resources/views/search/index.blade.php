@extends('layouts.noindex')

@php
if (isset($searchResult)) {
    $unescapedTitle = __('titles.search-result', ['result' => $titleKeyword]);
} else {
    $unescapedTitle = __('titles.search-index');
}
@endphp

@section('title', e($unescapedTitle))

@section('content')
<div class="container">
    <h1>{{ $unescapedTitle }}</h1>

    <form action="/search" method="get">
        <div class="pair">
            <label for="search-query">@lang('labels.search-query')</label>
            <input id="search-query" type="text" name="query" value="{{ $keyword ?: '' }}">
        </div>
        <div class="pairs">
            <div class="pair">
                <label for="search-type">@lang('labels.search-type')</label>
                <select id="search-type" name="type">
                    <option {!! kbs3_option('keyword', $selectedType) !!}>@lang('labels.search-keyword')</option>
                    <option {!! kbs3_option('tag', $selectedType) !!}>@lang('labels.search-tag')</option>
                    <option {!! kbs3_option('user', $selectedType) !!}>@lang('labels.search-user')</option>
                </select>
            </div>
            <div class="pair">
                <label for="search-sort">@lang('labels.posts-sort')</label>
                <select id="search-sort" name="sort">
                    <option {!! kbs3_option('created', $selectedSort) !!}>@lang('labels.sort-created')</option>
                    <option {!! kbs3_option('updated', $selectedSort) !!}>@lang('labels.sort-updated')</option>
                </select>
            </div>
        </div>
        <div class="pair">
            <input type="submit" value="送信">
        </div>
    </form>

    @if(isset($searchResult))
        @foreach($searchResult as $post)
            @component('components.post-block', ['post' => $post])
            @endcomponent
        @endforeach

        {{ $searchResult->links() }}
    @endif
</div>
@endsection
