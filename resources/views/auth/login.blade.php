@extends('layouts.noindex')

@section('title', 'ログイン')

@section('content')
<div class="container">
    <div class="medium box">
        <h1>ログイン</h1>
        <form action="{{ route('login') }}" method="post">
            @csrf
            <!-- Email -->
            <div class="pair">
                <label for="email"><i class="fas fa-envelope"></i> メールアドレス</label>
                <input type="email" name="email" id="email" required autofocus  autocomplete="off">
            </div>
            <!-- パスワード -->
            <div class="pair">
                <label for="password"><i class="fas fa-key"></i> パスワード</label>
                <input type="password" name="password" id="password" required autocomplete="off">
            </div>
            <!-- したまま -->
            <div class="checkbox pair">
                <input type="checkbox" name="remember" id="remember">
                <label for="remember">ログインしたままにする</label>
            </div>
            <!-- ログイン -->
            <div class="pair">
                <input type="submit" value="ログイン">
            </div>
            <div class="pair">
                <a href="{{ route('password.request') }}">パスワードを忘れてしまいました</a><br>
            </div>
        </form>
    </div>
</div>
@endsection
