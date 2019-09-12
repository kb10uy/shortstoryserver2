@extends('layouts.default')

@section('title', __('titles.series-edit'))

@section('content')
<div class="container" id="app">
    <h1>@lang('titles.series-edit')</h1>

    <form name="editseries" method="POST" action="{{ route('series.update', ['id' => $id]) }}" onsubmit="return false;">
        @csrf
        <input type="hidden" name="_method" value="PATCH">
        <div class="pair">
            <label for="title">@lang('labels.title')</label>
            <input type="text" name="title" id="title" value="{{ $title }}">
        </div>
        <div class="pair">
            <label for="body">@lang('labels.description')</label>
            <textarea name="description" id="description" cols="30" rows="3">{{ $description }}</textarea>
        </div>

        <hr>
        <div class="pair">
            <button type="button" class="button" onclick="editseries.submit();">@lang('actions.series-update')</button>
        </div>
    </form>
</div>
@endsection
