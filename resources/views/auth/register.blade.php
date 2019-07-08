@extends('layouts.noindex')

@section('title', __('titles.auth-signup'))

@section('content')
<div class="container">
    <div class="medium box">
        <h1>@lang('titles.auth-signup')</h1>
        <form id="signup-form" action="{{ route('register') }}" method="post" onsubmit="submitRegistration(); return false;">
            @csrf
            <input type="hidden" id="recaptcha" name="recaptcha_token">

            <!-- ユーザー名 -->
            <div class="pair">
                <label for="name"><i class="fas fa-user"></i> @lang('labels.username')</label>
                <input type="text" name="name" id="name" required autofocus  autocomplete="off">
            </div>
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
            <!-- パスワード再入力 -->
            <div class="pair">
                <label for="password-confirm"><i class="fas fa-key"></i> @lang('labels.password-confirm')</label>
                <input type="password" name="password_confirmation" id="password-confirm" required autocomplete="off">
            </div>

            <!-- サインアップ -->
            <div class="pair">
                <input id="submit-form" type="submit" value="{{ __('actions.auth-signup') }}">
            </div>
        </form>
    </div>
</div>

<!-- reCAPTCHA v3 -->
<script defer src="https://www.google.com/recaptcha/api.js?render={{ env('RECAPTCHA3_SITE_KEY') }}"></script>
<script defer>
function submitRegistration() {
    const form = document.querySelector('#signup-form');
    const submit = form.querySelector('#submit-form');
    const tokenHidden = form.querySelector('#recaptcha');
    const siteKey = '{{ env("RECAPTCHA3_SITE_KEY") }}';

    grecaptcha.ready(async () => {
        try {
            submit.disabled = true;
            const token = await grecaptcha.execute(siteKey, { action: 'homepage' });
            tokenHidden.value = token;
            form.submit();
        } catch(e) {
            console.log(e);
            submit.disabled = false;
        }
    });
}
</script>
@endsection
