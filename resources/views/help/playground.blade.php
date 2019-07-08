@extends('layouts.default')

@section('title', __('titles.help-playground'))

@section('content')
<div class="container">
    <h1>@lang('titles.help-playground')</h1>
    <p>
        ここでは <i>ShortStoryServer Writer's Format</i> を試すことができます。
    </p>
    <form action="{{ route('help.playground') }}" method="post" name="trys3wf2">
        @csrf
        <div class="pair">
            <label for="body">@lang('labels.body-text')</label>
            <textarea name="body" id="body" cols="30" rows="10">{{ old('body') }}</textarea>
        </div>
        <div class="pair">
            <button type="button" class="button" onclick="trys3wf2.submit();">@lang('actions.posts-submit')</button>
        </div>
    </form>

    <hr>
    <h2>パース結果</h2>
    {!! $parsedHtml !!}
</div>
@endsection

