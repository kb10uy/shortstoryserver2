@extends('layouts.noindex')

@section('title', __('titles.series-latest'))

@section('content')
<div class="container">
    <h1>@lang('titles.series-latest')</h1>

    @foreach($series as $seriesItem)
        @component('components.series-block', ['series' => $seriesItem])
        @endcomponent
    @endforeach

    {{ $series->links() }}
</div>
@endsection

