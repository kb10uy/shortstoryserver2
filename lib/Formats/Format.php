<?php

declare(strict_types=1);

namespace Lib\Formats;

abstract class Format
{
    abstract public function parse(string $source): void;
    abstract public function toHtml(): string;
}
