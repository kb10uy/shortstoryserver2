<!DOCTYPE html>
<html lang="{{ str_replace('_', '-', app()->getLocale()) }}">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <meta name="robots" content="noindex,nofollow,noarchive">
        <meta name="csrf-token" content="{{ csrf_token() }}">
        @include('components.includes')
        @yield('includes')
        <title>@yield('title') - ShortStoryServer</title>
    </head>
    <body>
        @include('components.navbar')
        @include('components.alerts')
        @yield('content')
        <script src="{{ mix('/scripts/vendor.js') }}"></script>
        <script src="{{ mix('/scripts/app.js') }}"></script>
    </body>
</html>
