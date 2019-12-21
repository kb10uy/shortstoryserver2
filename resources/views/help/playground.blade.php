@extends('layouts.default')

@section('title', __('titles.help-playground'))

@section('includes')
<script defer src="{{ mix('/scripts/playground.js') }}"></script>
@endsection

@section('content')
<div class="container">
    <h1>@lang('titles.help-playground')</h1>
    <p>
        ここでは <i>ShortStoryServer Writer's Format</i> を試すことができます。
        WebAssembly に対応したブラウザー環境が必要です。
    </p>
    <details>
        <summary>構文の説明 (クリックで展開)</summary>
        <article class="post">
            <h3>コマンド</h3>
            行頭に書く必要があります。
            <ul>
                <li>
                    <code>:character <i>gender</i> <i>id</i> <i>name</i></code> キャラクターの定義(ソース先頭を推奨)
                    <ul>
                        <li><strong>gender</strong> 性別の指定。 <code>male female mob</code> のいずれかか、カラーコードを指定</li>
                        <li><strong>id</strong> 文中で参照する ID</li>
                        <li><strong>name</strong> 実際に表示する名前</li>
                    </ul>
                </li>
                <li>
                    <code>:trim <i>type</i></code> 各行のトリミング挙動の変更
                    <ul>
                        <li><strong>type</strong> トリミングの挙動。 <code>never ascii unicode</code> のいずれかを指定。それぞれ「トリミングしない」、「ASCII 空白のみ」、「全ての空白」</li>
                    </ul>
                </li>
                <li>
                    <code>:autobr <i>type</i></code> 自動改行の挙動の変更
                    <ul>
                        <li><strong>type</strong> 自動改行をさせる場合は <code>always</code> 、させない場合(デフォルト)は <code>never</code> を指定する
                    </ul>
                </li>
            </ul>

            <h3>セリフ展開</h3>
            <ul>
                <li><code>@chara 「…」</code> 行セリフ。必ず改行が挿入されます。また行頭から記述する必要があります。キャラ ID の後にスペースを入れるのを忘れないように！</li>
                <li><code>[@chara 「…」]</code>インラインセリフ。同様に ID の後のスペースを忘れないようにしてください。</li>
            </ul>

            <h3>ブロック要素</h3>
            コマンド同様、行頭に書く必要があります。
            複数行に渡るブロックについては、<code>/foo &gt;&gt;&gt;</code> という行と <code>/foo &lt;&lt;&lt;</code> という行で内容を囲います。
            <ul>
                <li><code>/sec 見出し</code> 見出し。将来的にリンクが生成できるようになります</li>
                <li><code>/subsec 小見出し</code> 小見出し。将来的にリンクが生成できるようになります</li>
                <li><code>/para >>></code> 段落。あえて使う必要性はあまりありませんが……</li>
                <li><code>/quote >>></code> 引用</li>
                <li><code>/hori</code> 水平線</li>
            </ul>

            <h3>インライン要素</h3>
            文中の任意の場所で書けますが、ソース内で改行をまたぐことはできません。
            以下の要素は入れ子にして使うことができます。また、ブロック要素の中でももちろん利用できます。
            <ul>
                <li><code>[b 太字]</code> <strong>太字</strong></li>
                <li><code>[i 斜体]</code> <i>斜体</i></li>
                <li><code>[ul 下線]</code> <span class="underline">下線</span></li>
                <li><code>[st 取消線]</code> <del>取消線</del></li>
                <li><code>[dt 傍点]</code> <span class="dots">傍点</span></li>
                <li><code>[br]</code> 改行(ソース中の改行は反映されません)</li>
                <li><code>[link{https://example.com}リンク]</code> <a href="https://example.com">リンク</a></li>
                <li><code>[ruby{ふりがな}文章]</code> <ruby>文章<rp>(</rp><rt>ふりがな</rt><rp>)</rp></ruby></li>
            </ul>
        </article>
    </details>

    <hr>
    <div class="form-like" id="app">
        <div class="pair">
            <label for="body">@lang('labels.body-text')</label>
            <textarea id="body" cols="30" rows="10" v-model="source"></textarea>
        </div>
        <hr>
        <h2>パース結果</h2>
        <s3wf2-renderer :source="source" />
    </div>
</div>
@endsection

