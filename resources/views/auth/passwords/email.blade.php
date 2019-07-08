@extends('layouts.noindex')

@section('title', __('titles.auth-forgotpassword'))

@section('content')
<div class="container">
    <div class="medium box">
        <h1>@lang('titles.auth-forgotpassword')</h1>
        <p>
            パスワードをリセットするためのリンクを登録されているメールアドレスに送信します。
        </p>
        <form action="{{ route('password.email') }}" method="post">
            @csrf
            <!-- Email -->
            <div class="pair">
                <label for="email"><i class="fas fa-envelope"></i> @lang('labels.email')</label>
                <input type="email" name="email" id="email" required autofocus  autocomplete="off">
            </div>
            <div class="pair">
                <input id="submit-form" type="submit" value="{{ __('actions.auth-sendlink') }}">
            </div>
        </form>
    </div>
</div>
@endsection
