<?php
declare(strict_types=1);

namespace Lib\Formats;

use Illuminate\Support\Collection;

class Format
{
    public $nodes;

    public function __construct()
    {
        $this->nodes = collect([]);
    }

    public function toHtml(): string
    {
        $node = new Node;
    }
}
