<?php

declare(strict_types=1);

namespace Lib\Formats;

use Illuminate\Support\Collection;

class SingularNode extends Node
{
    /**
     * このノードの HTML を生成する。
     *
     * @return string 文字列
     */
    public function emit(): string
    {
        ob_start();
        // 開始タグ
        echo "<{$this->tagName}";
        foreach ($this->attributes as $name => $value) {
            $escaped = htmlspecialchars($value);
            echo " $name=\"$escaped\"";
        }
        echo ' />';

        return ob_get_clean() ?: '';
    }

    /**
     * string キャスト.
     */
    public function __toString(): string
    {
        return $this->emit();
    }
}
