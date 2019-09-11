@extends('layouts.noindex')

@section('title', htmlspecialchars(__('titles.series-edit-order', ['title' => $series->title])))

@section('content')
<div class="container">
    <h1>{{ __('titles.series-edit-order', ['title' => $series->title]) }}</h1>

    <table class="series-editor">
        <thead>
            <tr>
                <th>移動</th>
                <th>タイトル</th>
                <th>登録日時</th>
            </tr>
        </thead>
        <tbody>
            @foreach($posts as $post)
                <tr>
                    <td class="handle"><i class="fas fa-grip-lines"></i></td>
                    <td class="title">{{ $post->title }}</td>
                    <td class="date">{{ $post->pivot->created_at }}</td>
                </tr>
            @endforeach
        </tbody>
    </table>
</div>
@endsection

