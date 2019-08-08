<div class="profile">
    <img src="{{ Auth::user()->avatar_url }}" alt="avatar" class="avatar">
    <div class="username">{{ Auth::user()->name }}</div>
    <div class="screen-name">&#x40;{{ Auth::user()->name }}</div>
</div>
<ul class="menu">
    <li><a class="active" href="#">@lang('actions.menu-dashboard')</a></li>
    <li><a href="#">@lang('actions.menu-postsmanagement')</a></li>
    <li><a href="#">@lang('actions.menu-seriesmanagement')</a></li>
    <li><a href="#">@lang('actions.menu-bookmarksmanagement')</a></li>
    <li><a href="#">@lang('actions.menu-setting')</a></li>
    <li><a href="#">@lang('actions.auth-logout')</a></li>
</ul>
