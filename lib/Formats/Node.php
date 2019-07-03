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

    public function __construct(string $tagName, Collection $attributes = null)
    {
        $this->tagName = $tagName;
        $this->attributes = $attributes ?? collect();
        $this->children = collect();
    }

    /**
     * このノードの HTML を生成する。
     *
     * @return HTML 文字列
     */
    public function emit(): string
    {
    }
}
