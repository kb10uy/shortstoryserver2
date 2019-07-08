@extends('layouts.default')

@section('title', __('titles.help-about'))

@section('content')
<div class="container">
    <h1>ShortStoryServer について</h1>
    <p>
        ShortStoryServer は、よくわからない SS (ショートストーリー) や怪文書を投稿するためのサイトです。
        2015 年ごろに運用を開始し、フレームワーク・バックエンドを転々としつつなんだかんだ <strong>4 年目</strong>に突入してしまいました。
        <small>どうしてくれんねん</small>
    </p>
    <p>
        名付けて、 <i>kb10uy ShortStoryServer v2, kbS32</i> です。
        <small>C:\Windows\System32\kbs32.dll ではない</small>
    </p>

    <h2>特長</h2>
    <h3>SS に特化した構文をサポート</h3>
    <p>
        <code>/\dちゃんねる/</code> の SS などでよく見られる、<code>/(.+)「.+」/</code> 形式のセリフに対応した <i>ShortStoryServer Writer's Format v2</i> を用意しています。
        <code>@someone</code> のようにキャラクターを指定することで自動で色付けを行ってくれるほか、今までは手動で色を決定していた部分を改良し、あらかじめ用意された
        いくつかの色の中から自動的に決定されるようになりました。
    </p>

    <h3>不安定なデータベース管理</h3>
    <p>
        それは特徴であって特長じゃないだろって？うるせえ
    </p>
</div>
@endsection

