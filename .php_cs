<?php

$finder = PhpCsFixer\Finder::create()
    ->exclude('./resources/views')
    ->exclude('./storage')
    ->in(__DIR__);

return PhpCsFixer\Config::create()
    ->setRules([ '@PSR2' => true ])
    ->setFinder($finder);
