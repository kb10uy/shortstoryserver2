@extends('layouts.noindex')

@section('title', __('titles.auth-login'))

@section('content')
<div class="container">
    <div class="medium box">
        <h1>@lang('titles.auth-login')</h1>
        <form action="{{ route('login') }}" method="post">
            @csrf
            <!-- Email -->
            <div class="pair">
                <label for="email"><i class="fas fa-envelope"></i> @lang('labels.email')</label>
                <input type="email" name="email" id="email" required autofocus  autocomplete="off">
            </div>
            <!-- パスワード -->
            <div class="pair">
                <label for="password"><i class="fas fa-key"></i> @lang('labels.password')</label>
                <input type="password" name="password" id="password" required autocomplete="off">
            </div>
            <!-- したまま -->
            <div class="checkbox pair">
                <input type="checkbox" name="remember" id="remember">
                <label for="remember">@lang('labels.rememberme')</label>
            </div>
            <!-- ログイン -->
            <div class="pair">
                <input type="submit" value="{{ __('actions.auth-login') }}">
            </div>
            <div class="pair">
                <a href="{{ route('password.request') }}">@lang('labels.forgot-password')</a><br>
            </div>
        </form>
    </div>
</div>
@endsection
