<section class="post-summary">
    <h2>
        <a href="{{ route('posts.show', ['id' => $post->id]) }}">{{ $post->title }}</a>
        <small>by {{ $post->user->name }}</small>
    </h2>
    <p class="summary">
        @if($post->description)
            {{ $post->description }}
        @else
            @lang('labels.no-description')
        @endif
    </p>
    @if($post->tags->isNotEmpty())
        <ul class="tags">
            @foreach($post->tags as $tag)
                <li class="tag"><a href="{{ route('search.tag', ['q' => $tag->name]) }}">{{ $tag->name }}</a></li>
            @endforeach
        </ul>
    @endif
</section>
