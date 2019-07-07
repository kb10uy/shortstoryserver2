<?php

declare(strict_types=1);

namespace Lib\Formats\S3wf2;

use Lib\Formats\Node;
use Lib\Formats\ParseErrorException;

/**
 * キャラクター全員を扱う.
 */
class CharacterSet
{
    public const DEFAULT_COLORS_MAX = 4;

    private $characters;
    private $customColors;
    private $maleIndex = 0;
    private $femaleIndex = 0;
    private $mobIndex = 0;

    public function __construct()
    {
        $this->characters = collect();
        $this->customColors = collect();
    }

    /**
     * 新しいキャラクターを設定する。
     *
     * @param string $key  参照名
     * @param string $type キャラクターのタイプ。 male female mob のいずれかか、カラーコードを指定
     * @param string $name 表示される名前
     */
    public function set(string $key, string $type, string $name): void
    {
        switch ($type) {
            case 'male':
                $setType = 'male' . ($this->maleIndex + 1);
                ++$this->maleIndex;
                break;
            case 'female':
                $setType = 'female' . ($this->femaleIndex + 1);
                ++$this->femaleIndex;
                break;
            case 'mob':
                $setType = 'mob' . ($this->mobIndex + 1);
                ++$this->mobIndex;
                break;
            default:
                $isColorCode = preg_match('/^#([\da-f]{3,6})$/ui', $type, $matches);
                if (1 !== $isColorCode) {
                    throw new ParseErrorException("Invalid color code: $type");
                }
                $this->customColors->push($matches[1]);
                $setType = $matches[1];
                break;
        }
        $this->characters[$key] = new Character($name, $setType);
    }

    /**
     * 指定したキーのキャラクターを取得する。
     *
     * @param string $key キー
     *
     * @return Character 存在する場合はそのキャラクター、ない場合は null
     */
    public function get(string $key): ?Character
    {
        return $this->characters->has($key) ? $this->characters[$key] : null;
    }

    /**
     * カスタムキャラクターカラーの style タグの Node を生成する。
     *
     * @return Node
     */
    public function generateCustomColorsStyle(): Node
    {
        $node = new Node('style');
        $node->addTextNode(PHP_EOL);
        foreach ($this->customColors as $color) {
            $node->addTextNode(".custom-$color { color: #$color; }" . PHP_EOL);
        }

        return $node;
    }
}
