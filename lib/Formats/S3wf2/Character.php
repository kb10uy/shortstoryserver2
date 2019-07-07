<?php

declare(strict_types=1);

namespace Lib\Formats\S3wf2;

/**
 * キャラクターの情報.
 */
class Character
{
    private $displayName;
    private $colorClass;

    public function __construct(string $name, string $color)
    {
        $this->displayName = $name;
        $this->colorClass = $color;
    }

    /**
     * セリフで表示される名前.
     *
     * @return string
     */
    public function displayName(): string
    {
        return $this->displayName;
    }

    /**
     * このキャラクターに割り当てられたクラス.
     *
     * @return string
     */
    public function colorClass(): string
    {
        return $this->colorClass;
    }
}
