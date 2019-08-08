<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Lib\Formats\S3wf2\S3wf2Format;

class HelpController extends Controller
{
    public function about()
    {
        return view('help.about');
    }

    public function terms()
    {
        return view('help.terms');
    }

    public function playground(Request $request)
    {
        $parsedHtml = '';
        if ($request->filled('body')) {
            $request->flash();

            $format = new S3wf2Format();
            $format->parse($request->body);
            $parsedHtml = $format->toHtml();
        }

        return view('help.playground', compact('parsedHtml'));
    }
}
