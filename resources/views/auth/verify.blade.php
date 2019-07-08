@extends('layouts.app')

@section('title', __('titles.auth-sentverification'))

@section('content')
<div class="container">
    @if (session('resent'))
        <div class="alert alert-success" role="alert">
            {{ __('A fresh verification link has been sent to your email address.') }}
        </div>
    @endif

    <h1>@lang('titles.auth-sentverification')</h1>
    <p>
        メールアドレスの有効性を確認するため、入力されたメールアドレスに確認用のリンクを送信しました。
        この画面を閉じる前に、メールをご確認ください。
    </p>
    <a href="{{ route('verification.resend') }}">@lang('actions.auth-resend')</a>
</div>
@endsection
