<div class="profile">
    <img src="{{ Auth::user()->avatar_url }}" alt="avatar" class="avatar">
    <div class="username">{{ Auth::user()->name }}</div>
    <div class="screen-name">&#x40;{{ Auth::user()->name }}</div>
</div>
<ul class="menu">
    <li><a class="{{ $selected === 'dashboard' ? 'active' : '' }}" href="{{ route('dashboard.index') }}">@lang('actions.menu-dashboard')</a></li>
    <li><a class="{{ $selected === 'posts' ? 'active' : '' }}" href="{{ route('dashboard.posts') }}">@lang('actions.menu-postsmanagement')</a></li>
    <li><a class="{{ $selected === 'series' ? 'active' : '' }}" href="#">@lang('actions.menu-seriesmanagement')</a></li>
    <li><a class="{{ $selected === 'bookmarks' ? 'active' : '' }}" href="#">@lang('actions.menu-bookmarksmanagement')</a></li>
    <li><a class="{{ $selected === 'setting' ? 'active' : '' }}" href="#">@lang('actions.menu-setting')</a></li>
    <li><a href="{{ route('logout') }}" onclick="document.getElementById('logout').submit(); return false;">@lang('actions.auth-logout')</a></li>
</ul>
