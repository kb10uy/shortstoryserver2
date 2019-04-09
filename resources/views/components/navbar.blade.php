<header>
    <nav class="navbar">
        <span class="logo"><a href="/" class="item">ShortStoryServer</a></span>
        <div class="menu">
            <a href="/" class="item">一覧</a>
            <a href="/" class="item">検索</a>
        </div>

        @auth
            <div class="user" data-dropdown="user-dropdown">
                <img src="http://placehold.jp/256x256.png" alt="username" class="avatar">
                <span class="dropdown-caret"></span>

                <!-- メニュー -->
                <div id="user-dropdown" class="dropdown">
                    <div class="info">
                        Signed in as<br>
                        <strong>{{ Auth::user()->name }}</strong>
                    </div>
                    <div class="separator"></div>
                    <a href="{{ route('dashboard.index') }}" class="item">ダッシュボード</a>
                    <a href="/" class="item">作品</a>
                    <a href="/" class="item">設定</a>
                    <a href="{{ route('logout') }}" class="item"
                        onclick="document.getElementById('logout').submit(); return false;">
                        ログアウト
                    </a>
                </div>
            </div>
            <form id="logout" action="{{ route('logout') }}" method="post">
                @csrf
            </form>
        @endauth
        @guest
            <div class="user" data-dropdown="user-dropdown">
                <span>ログインしていません</span>
                <span class="dropdown-caret"></span>

                <!-- メニュー -->
                <div id="user-dropdown" class="dropdown">
                    <div class="info">
                        ログインして様々な機能を活用しましょう
                    </div>
                    <div class="separator"></div>
                    <a href="{{ route('login') }}" class="item">ログイン</a>
                    <a href="{{ route('register') }}" class="item">サインアップ</a>
                </div>
            </div>
        @endguest
    </nav>
</header>
