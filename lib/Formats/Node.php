<?php

declare(strict_types=1);

namespace Lib\Formats;

use Illuminate\Support\Collection;

class Node
{
    /** @var string タグ */
    private $tagName;

    /** @var Collection 属性 */
    private $attributes;

    /** @var Collection 子要素 */
    private $children;

    /**
     * @param $tagName タグ名
     * @param $attributes タグに付与する属性
     */
    public function __construct(string $tagName, Collection $attributes = null)
    {
        $this->tagName = $tagName;
        $this->attributes = $attributes ?? collect();
        $this->children = collect();
    }

    /**
     * 文字列.
     */
    public function addTextNode(string $node): void
    {
        $node = trim($node);
        if ('' === $node) {
            return;
        }
        $this->children->push(htmlspecialchars($node));
    }

    /**
     * 子ノード.
     */
    public function addNode(Node $node): void
    {
        $this->children->push($node);
    }

    /**
     * このノードの HTML を生成する。
     *
     * @return HTML 文字列
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
        echo '>';

        // コンテンツ
        foreach ($this->children as $child) {
            echo (string) $child;
        }

        // 終了タグ
        echo "</{$this->tagName}>";

        return ob_get_clean();
    }

    /**
     * string キャスト.
     */
    public function __toString(): string
    {
        return $this->emit();
    }
}
