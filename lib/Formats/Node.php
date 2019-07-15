<?php

declare(strict_types=1);

namespace Lib\Formats;

use Illuminate\Support\Collection;

class Node
{
    /** @var string タグ */
    protected $tagName;

    /** @var Collection 属性 */
    protected $attributes;

    /** @var Collection パラメーター */
    protected $parameters;

    /** @var Collection 子要素 */
    protected $children;

    /** @var callable カスタムエミッター */
    protected $emitter;

    /**
     * @param string     $tagName    タグ名
     * @param Collection $attributes タグに付与する属性
     */
    public function __construct(string $tagName, Collection $attributes = null)
    {
        $this->tagName = $tagName;
        $this->attributes = $attributes ?? collect();
        $this->children = collect();
        $this->parameters = collect();
    }

    /**
     * タグ名
     */
    public function tagName(): string
    {
        return $this->tagName;
    }

    /**
     * 文字列.
     */
    public function addTextNode(string $node): void
    {
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
     * パラメーター
     */
    public function addParameter(Node $parameter): void
    {
        $this->parameters->push($parameter);
    }

    /**
     * エミッターを指定する。
     * シグネチャは function (string $tagName, Collection $attributes, Collection $parameters, Collection $children)
     */
    public function setEmitter(callable $emitter): void
    {
        $this->emitter = $emitter;
    }

    /**
     * このノードの HTML を生成する。
     *
     * @return string 文字列
     */
    public function emit(): string
    {
        if ($this->emitter !== null) {
            return call_user_func($this->emitter, $this->tagName, $this->attributes, $this->parameters, $this->children);
        }

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

        return ob_get_clean() ?: '';
    }

    /**
     * プレーンテキストでこのノードを生成する。
     *
     * @return string 文字列
     */
    public function emitPlain(): string
    {
        return htmlspecialchars_decode(strip_tags($this->emit()));
    }

    /**
     * string キャスト.
     */
    public function __toString(): string
    {
        return $this->emit();
    }
}
