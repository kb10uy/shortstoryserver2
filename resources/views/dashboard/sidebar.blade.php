<div class="profile">
    <img src="http://placehold.jp/256x256.png" alt="avatar" class="avatar">
    <div class="username">{{ Auth::user()->name }}</div>
    <div class="screen-name">&#x40;{{ Auth::user()->name }}</div>
</div>
<ul class="menu">
    <li><a class="active" href="#">ダッシュボード</a></li>
    <li><a href="#">投稿管理</a></li>
    <li><a href="#">シリーズ管理</a></li>
    <li><a href="#">ブックマーク管理</a></li>
    <li><a href="#">設定</a></li>
    <li><a href="#">ログアウト</a></li>
</ul>
