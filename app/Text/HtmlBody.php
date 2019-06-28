<?php

namespace App\Text;

/**
 * 本文の HTML のメタ情報を表す。
 */
class HtmlBody
{
    /** @var string 元テキストの種別 */
    private $type;

    /** @var array 見出し */
    private $headers;

    /** @var string 本文HTML */
    private $body;

    public function __construct(string $type, array $headers, string $body)
    {
        $this->type = $type;
        $this->headers = $headers;
        $this->body = $body;
    }
}
