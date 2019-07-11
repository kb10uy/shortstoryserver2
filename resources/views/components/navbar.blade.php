<header>
    <nav class="navbar">
        <span class="logo"><a href="/" class="item">ShortStoryServer</a></span>
        <div class="menu">
            <a href="{{ route('posts.latest') }}" class="item">@lang('actions.menu-list')</a>
            <a href="#" class="item">@lang('actions.menu-search')</a>
            <a class="item" data-dropdown="help-dropdown">@lang('actions.menu-help')</a>
            <div id="help-dropdown" class="dropdown" data-dropdown-merge>
                <a href="{{ route('help.playground') }}" class="item">@lang('actions.help-playground')</a>
                <a href="{{ route('help.about') }}" class="item">@lang('actions.help-about')</a>
                <a href="{{ route('help.terms') }}" class="item">@lang('actions.help-terms')</a>
            </div>
        </div>

        @auth
            <div class="user" data-dropdown="user-dropdown">
                <img src="https://via.placeholder.com/64" alt="{{ Auth::user()->name }}" class="avatar">
                <span class="dropdown-caret"></span>

                <!-- メニュー -->
                <div id="user-dropdown" class="dropdown">
                    <div class="info">
                        @lang('labels.login-as')<br>
                        <strong>{{ Auth::user()->name }}</strong>
                    </div>
                    <div class="separator"></div>
                    <a href="{{ route('dashboard.index') }}" class="item">@lang('actions.menu-dashboard')</a>
                    <a href="/" class="item">@lang('actions.menu-posts')</a>
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
