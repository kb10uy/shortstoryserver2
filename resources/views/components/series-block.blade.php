<section class="series-summary">
    <h2>
        <a href="{{ route('series.show', ['id' => $series->id]) }}">{{ $series->title }}</a>
        <small>by {{ $series->user->name }}</small>
    </h2>
    <p class="summary">
        @if($series->description)
            {{ $series->description }}
        @else
            @lang('labels.no-description')
        @endif
    </p>
</section>
