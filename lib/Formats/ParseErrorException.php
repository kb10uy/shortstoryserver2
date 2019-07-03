<?php

declare(strict_types=1);

namespace Lib\Formats;

use Exception;

class ParseErrorException extends Exception
{
    private $errorLine;

    public function __construct(int $line, string $message, Exception $previous = null)
    {
        parent::__construct($message, 0, $previous);
        $this->errorLine = $line;
    }

    public function __toString(): string
    {
        return __CLASS__.": Parse failed at line {$this->line}";
    }
}
