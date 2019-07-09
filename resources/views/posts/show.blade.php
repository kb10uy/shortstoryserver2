@extends('layouts.default')

@section('title', $title)

@section('content')
<div class="container">
    <h1>{{ $title }}</h1>
    {!! $articleHtml !!}
</div>
@endsection
