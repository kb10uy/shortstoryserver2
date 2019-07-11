<div class="container" id="statuses">
    @if (session('status'))
        <div class="alert success" role="alert">
            <i class="fas fa-info"></i>
            <span class="text">{{ session('status') }}</span>
        </div>
    @endif
    @if (session('error'))
        <div class="alert error" role="alert">
            <i class="fas fa-times"></i>
            <span class="text">{{ session('error') }}</span>
        </div>
    @endif
    @foreach ($errors->all() as $error)
        <div class="message error">
            <i class="fas fa-times"></i>
            <span class="text">{{ $error }}</span>
        </div>
    @endforeach
</div>
