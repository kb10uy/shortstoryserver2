<?php

declare(strict_types=1);

namespace Lib\Formats;

use Exception;

class ParseErrorException extends Exception
{
    private $errorLine;

    public function __construct(string $message, int $line = -1, Exception $previous = null)
    {
        parent::__construct($message, 0, $previous);
        $this->errorLine = $line;
    }

    public function __toString(): string
    {
        return __CLASS__ . ": Parse failed at line {$this->line} ({$this->getMessage()})";
    }
}
