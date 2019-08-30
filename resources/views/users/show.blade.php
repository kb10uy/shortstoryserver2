@extends('layouts.default')

@section('title', htmlspecialchars(__('titles.users-profile', ['name' => $user->name])))

@section('includes')
<script defer src="{{ mix('/scripts/show-user.js') }}"></script>
<link rel="stylesheet" href="{{ mix('/styles/show-user.css') }}" media="all">
@endsection

@section('content')
<div class="container" id="app">
    <user-profile name="{{ $user->name }}" avatar="{{ $user->avatar_url }}">
        <template v-slot:posts-count>{{ $postsCount }}</template>
        <template v-slot:series-count>0</template>
        <template v-slot:bookmarks-count>0</template>
    </user-profile>
</div>
@endsection

