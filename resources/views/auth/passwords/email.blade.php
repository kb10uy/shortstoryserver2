@extends('layouts.noindex')

@section('content')
<div class="container">
    <div class="medium box">
        <h1>パスワードを忘れてしまいました</h1>
        <p>
            パスワードをリセットするためのリンクを登録されているメールアドレスに送信します。
        </p>
        <form action="{{ route('password.email') }}" method="post">
            @csrf
            <!-- Email -->
            <div class="pair">
                <label for="email"><i class="fas fa-envelope"></i> メールアドレス</label>
                <input type="email" name="email" id="email" required autofocus  autocomplete="off">
            </div>
            <div class="pair">
                <input id="submit-form" type="submit" value="リンクを送信">
            </div>
        </form>
    </div>
</div>
@endsection
