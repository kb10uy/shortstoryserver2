<?php

declare(strict_types=1);

namespace Lib\Formats\S3wf2;

use Lib\Formats\Format;
use Lib\Formats\Node;

class S3wf2Format extends Format
{
    public function parse(string $source): Node
    {
        $sourceLines = preg_split('/\r\n|\r|\n/', $source, -1, PREG_SPLIT_NO_EMPTY) ?: [];

        $paragraphReady = true;
        $currentText = '';
        foreach ($sourceLines as $lineNumber => $lineString) {
            if (0 === strpos($lineString, '//')) {
                continue;
            }

            if (false !== preg_match('/^(:|\/|@)(\w+)\s+(.+)$/u', $lineString, $matches)) {
                // escaped line
                switch ($matches[1]) {
                    case ':':
                        break;
                    case '/':
                        break;
                    case '@':
                        break;
                }
            } else {
                // normal line
            }
        }
    }
}
