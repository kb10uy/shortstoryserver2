<?php

declare(strict_types=1);

namespace Lib\Formats\S3wf2;

use Lib\Formats\ParseErrorException;

class State
{
    public const MALE_COLORS = [];
    public const FEMALE_COLORS = [];
    public const MOB_COLORS = [];

    private $maleIndex = 0;
    private $femaleIndex = 0;
    private $mobIndex = 0;

    private $characters;
    private $paragraphReady = true;
    private $currentText = '';

    public function __construct()
    {
        $this->characters = collect();
    }

    /**
     * キャラクターを追加する。
     *
     * @param string $type male, female, #ffffff のいずれか
     * @param string $key  参照する名前
     * @param string $name 表示する名前
     */
    public function setCharacter(string $type, string $key, string $name): void
    {
        $color = '';
        switch ($type) {
            case 'male':
                $color = MALE_COLORS[$this->maleIndex];
                ++$this->maleIndex;
                break;
            case 'female':
                $color = MALE_COLORS[$this->maleIndex];
                ++$this->maleIndex;
                break;
            case 'mob':
                $color = MALE_COLORS[$this->maleIndex];
                ++$this->maleIndex;
                break;
            default:
                if (false === preg_match('/^#([\da-f]){3,6}$/i', $type, $matches)) {
                    throw new ParseErrorException('Invalid character type. One of "male", "female" and "mob" or color code acceptable.');
                }
                $color = $type;
                break;
        }

        $this->characters[$key] = [$name, $color];
    }
}
