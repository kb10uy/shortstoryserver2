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
                $setType = 'male-' . ($this->maleIndex + 1);
                ++$this->maleIndex;
                break;
            case 'female':
                $setType = 'female-' . ($this->femaleIndex + 1);
                ++$this->femaleIndex;
                break;
            case 'mob':
                $setType = 'mob-' . ($this->mobIndex + 1);
                ++$this->mobIndex;
                break;
            default:
                $isColorCode = preg_match('/^#([\da-f]{3,6})$/ui', $type, $matches);
                if (1 !== $isColorCode) {
                    throw new ParseErrorException("Invalid color code: $type");
                }
                $this->customColors->push($matches[1]);
                $setType = "custom-{$matches[1]}";
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

        $node->addTextNode('@media (prefers-color-scheme: dark) {');
        foreach ($this->customColors as $color) {
            $inverted = $this->invertColorCode($color);
            $node->addTextNode("  .custom-$color { color: #$inverted; }" . PHP_EOL);
        }
        $node->addTextNode('}');

        return $node;
    }

    /**
     * ダークモード用に、明度を反転する。
     *
     * @return string 反転したカラーコード
     */
    private function invertColorCode(string $color): string
    {
        $original = 6 === strlen($color) ? $color : "{$color[0]}{$color[0]}{$color[1]}{$color[1]}{$color[2]}{$color[2]}";
        $red = hexdec(substr($original, 0, 2)) / 255.0;
        $green = hexdec(substr($original, 2, 2)) / 255.0;
        $blue = hexdec(substr($original, 4, 2)) / 255.0;

        [$hue, $saturation, $brightness] = $this->convertRgbToHsb([$red, $green, $blue]);
        [$red, $green, $blue] = $this->convertHsbToRgb([$hue, $saturation, $brightness]);

        $f2h = function (float $value) {
            $intValue = (int) ($value * 255.0);

            return str_pad(dechex($intValue), 2, '0');
        };

        return $f2h($red) . $f2h($green) . $f2h($blue);
    }

    private function convertRgbToHsb(array $rgbFloat): array
    {
        [$red, $green, $blue] = $rgbFloat;
        $max = $red > $green ? $red : $green;
        $max = $blue > $max ? $blue : $max;
        $min = $red < $green ? $red : $green;
        $min = $blue < $min ? $blue : $min;

        $hue = $max - $min;
        if ($hue > 0.0) {
            if ($max === $red) {
                $hue = ($green - $blue) / $hue;
                if ($hue < 0.0) {
                    $hue += 6.0;
                }
            } elseif ($max === $green) {
                $hue = 2.0 + ($blue - $red) / $hue;
            } else {
                $hue = 4.0 + ($red - $green) / $hue;
            }
        }
        $hue /= 6.0;

        $saturation = $max - $min;
        if (0.0 != $max) {
            $saturation /= $max;
        }

        $brightness = $max;

        return [$hue, $saturation, $brightness];
    }

    public function convertHsbToRgb(array $hsbFloat): array
    {
        [$hue, $saturation, $brightness] = $hsbFloat;
        [$red, $green, $blue] = [$brightness, $brightness, $brightness];

        if ($saturation > 0.0) {
            $hue *= 6.0;
            $area = (int) $hue;
            $rest = $hue - (float) $area;

            switch ($area) {
                case 0:
                    $green *= 1.0 - $saturation * (1.0 - $rest);
                    $blue *= 1.0 - $saturation;
                    break;
                case 1:
                    $red *= 1.0 - $saturation * $rest;
                    $blue *= 1.0 - $saturation;
                    break;
                case 2:
                    $red *= 1.0 - $saturation;
                    $blue *= 1.0 - $saturation * (1.0 - $rest);
                    break;
                case 3:
                    $red *= 1.0 - $saturation;
                    $green *= 1.0 - $saturation * $rest;
                    break;
                case 4:
                    $red *= 1.0 - $saturation * (1.0 - $rest);
                    $green *= 1.0 - $saturation;
                    break;
                case 5:
                    $green *= 1.0 - $saturation;
                    $blue *= 1.0 - $saturation * $rest;
                    break;
            }
        }

        return [$red, $green, $blue];
    }
}
