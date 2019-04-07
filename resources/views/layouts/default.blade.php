<!DOCTYPE html>
<html lang="{{ str_replace('_', '-', app()->getLocale()) }}">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        @include('components.includes')
        <title>@yield('title') - ShortStoryServer</title>
    </head>
    <body>
        @include('components.navbar')
        @yield('content')
    </body>
</html>
