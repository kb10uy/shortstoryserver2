<header>
    <nav class="navbar">
        <a class="logo" href="/">
            <img src="/images/newlogo.png" alt="kbS3">
            <span class="title">ShortStoryServer</span>
        </span>
        <div class="menu">
            <a class="item" data-dropdown="list-dropdown">@lang('actions.menu-list')</a>
            <div id="list-dropdown" class="dropdown" data-dropdown-merge>
                <a class="item" href="{{ route('posts.latest') }}">@lang('actions.posts-latest')</a>
                <a class="item" href="{{ route('series.latest') }}">@lang('actions.series-latest')</a>
            </div>

            <a href="{{ route('search.index') }}" class="item">@lang('actions.menu-search')</a>

            <a class="item" data-dropdown="help-dropdown">@lang('actions.menu-help')</a>
            <div id="help-dropdown" class="dropdown" data-dropdown-merge>
                <a href="{{ route('help.playground') }}" class="item">@lang('actions.help-playground')</a>
                <a href="{{ route('help.about') }}" class="item">@lang('actions.help-about')</a>
                <a href="{{ route('help.terms') }}" class="item">@lang('actions.help-terms')</a>
            </div>
        </div>

        @auth
            <div class="user" data-dropdown="user-dropdown">
                <img src="{{ Auth::user()->avatar_url }}" alt="{{ Auth::user()->name }}" class="avatar">
                <span class="dropdown-caret"></span>

                <!-- メニュー -->
                <div id="user-dropdown" class="dropdown">
                    <div class="info">
                        @lang('labels.login-as')<br>
                        <strong>{{ Auth::user()->name }}</strong>
                    </div>
                    <div class="separator"></div>
                    <a href="{{ route('dashboard.index') }}" class="item">@lang('actions.menu-dashboard')</a>
                    <a href="{{ route('posts.new') }}" class="item">@lang('actions.menu-posts')</a>
                    <a href="/" class="item">@lang('actions.menu-setting')</a>
                    <a href="{{ route('logout') }}" class="item"
                        onclick="document.getElementById('logout').submit(); return false;">
                        @lang('actions.auth-logout')
                    </a>
                </div>
            </div>
            <form id="logout" action="{{ route('logout') }}" method="post">
                @csrf
            </form>
        @endauth
        @guest
            <div class="user" data-dropdown="user-dropdown">
                <span>@lang('labels.logout')</span>
                <span class="dropdown-caret"></span>

                <!-- メニュー -->
                <div id="user-dropdown" class="dropdown">
                    <div class="info">
                        ログインして様々な機能を活用しましょう
                    </div>
                    <div class="separator"></div>
                    <a href="{{ route('login') }}" class="item">@lang('actions.auth-login')</a>
                    <a href="{{ route('register') }}" class="item">@lang('actions.auth-signup')</a>
                </div>
            </div>
        @endguest
    </nav>
</header>
