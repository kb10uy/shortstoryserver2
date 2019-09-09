@extends('layouts.noindex')

@section('title', __('titles.dashboard-top'))

@section('content')
<div class="container">
    <div class="dashboard">
        <div class="left">
            @component('dashboard.sidebar', ['selected' => 'dashboard'])
            @endcomponent
        </div>
        <div class="right">
            <div class="box">
                <h1>@lang('titles.dashboard-top')</h1>

                <!-- 作品 -->
                <h2>@lang('labels.new-post')</h2>
                <p>
                    新しい怪文書の着想を獲ましたか？さっそく公開してみましょう！
                    大丈夫ですよ！ モデレーターは定期的に誤操作で全てのデータを予告をなく削除しますので！
                </p>
                <a href="{{ route('posts.new') }}" class="button">@lang('actions.dashboard-newpost')</a>

                <hr>

                <!-- シリーズ -->
                <h2>@lang('labels.new-series')</h2>
                <p>
                    続編も投稿したのですか？いいですね！シリーズ機能を活用して読者に対するナビゲーションを効率化しましょう！
                </p>
                <a href="{{ route('series.new') }}" class="button">@lang('actions.dashboard-newseries')</a>

                <hr>

                <!-- ブックマーク -->
                <h2>@lang('labels.new-bookmark')</h2>
                <p>
                    投稿するだけでは飽き足らず、ついに他人の性癖にまで手を出すというんですね？結構！
                    kbs3 のブックマーク機能はあなたの心と下半身を全面的にサポートします。
                </p>
                <button class="button" disabled>@lang('actions.dashboard-newbookmark')</button>
            </div>
        </div>
    </div>
</div>
@endsection
