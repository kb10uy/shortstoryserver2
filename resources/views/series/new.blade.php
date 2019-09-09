@extends('layouts.default')

@section('title', __('titles.series-new'))

@section('content')
<div class="container" id="app">
    <h1>@lang('titles.series-new')</h1>

    <form name="newseries" method="POST" action="{{ route('series.post') }}" onsubmit="return false;">
        @csrf
        <div class="pair">
            <label for="title">@lang('labels.title')</label>
            <input type="text" name="title" id="title">
        </div>
        <div class="pair">
            <label for="body">@lang('labels.description')</label>
            <textarea name="description" id="description" cols="30" rows="3"></textarea>
        </div>

        <hr>
        <div class="pair">
            <button type="button" class="button" onclick="newseries.submit();">@lang('actions.series-submit')</button>
        </div>
    </form>
</div>
@endsection
