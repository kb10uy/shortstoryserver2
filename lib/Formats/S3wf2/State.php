<?php

declare(strict_types=1);

namespace Lib\Formats\S3wf2;

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
     * @param string $key 参照する名前
     * @param string $name 表示する名前
     */
    public function setCharacter(string $type, string $key, string $name)
    {
        $color = '';
        switch ($type) {
            case 'male':

            break;
            case 'female':
            break;
            case 'mob':
            break;
            default:
            break;
        }
    }
}
