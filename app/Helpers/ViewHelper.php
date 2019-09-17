<?php

function kbs3_option(string $value, ?string $selected)
{
    $attr = "value=\"$value\"";

    return $value === $selected ? $attr . ' selected' : $attr;
}
