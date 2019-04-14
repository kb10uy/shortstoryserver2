@extends('layouts.noindex')

@section('title', 'ダッシュボード')

@section('content')
<div class="container">
    <div class="dashboard">
        <div class="left">
            @include('dashboard.sidebar')
        </div>
        <div class="right">
            <div class="box">
                <h1>ダッシュボード</h1>

                <!-- 作品 -->
                <h2>新しい作品</h2>
                <p>
                    新しい怪文書の着想を獲ましたか？さっそく公開してみましょう！
                    大丈夫ですよ！ モデレーターは定期的に誤操作で全てのデータを予告をなく削除しますので！
                </p>
                <a href="{{ route('posts.new') }}" class="button">新しい作品を投稿する</a>

                <hr>

                <!-- シリーズ -->
                <h2>新しいシリーズ</h2>
                <p>
                    続編も投稿したのですか？いいですね！シリーズ機能を活用して読者に対するナビゲーションを効率化しましょう！
                </p>
                <button class="button" disabled>新しい作品を投稿する</button>

                <hr>

                <!-- ブックマーク -->
                <h2>新しいブックマーク</h2>
                <p>
                    投稿するだけでは飽き足らず、ついに他人の性癖にまで手を出すというんですね？結構！
                    kbs3 のブックマーク機能はあなたの心と下半身を全面的にサポートします。
                </p>
                <button class="button" disabled>新しいブックマークリストを作成</button>
            </div>
        </div>
    </div>
</div>
@endsection
