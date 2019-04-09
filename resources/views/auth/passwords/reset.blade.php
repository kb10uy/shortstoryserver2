@extends('layouts.noindex')

@section('content')
<div class="container">
    <div class="medium box">
        <h1>パスワードのリセット</h1>
        <p>
            登録しているメールアドレス、及び新しいパスワードを入力してください。
        </p>
        <form action="{{ route('password.email') }}" method="post">
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
            <!-- パスワード再入力 -->
            <div class="pair">
                <label for="password-confirm"><i class="fas fa-key"></i> パスワード(再入力)</label>
                <input type="password" name="password_confirmation" id="password-confirm" required autocomplete="off">
            </div>
            <div class="pair">
                <input id="submit-form" type="submit" value="パスワードをリセット">
            </div>
        </form>
    </div>
</div>
@endsection
