<?php

$finder = PhpCsFixer\Finder::create()
    ->exclude('./resources/views/**/*')
    ->exclude('./storage/**/*')
    ->in(__DIR__);

return PhpCsFixer\Config::create()
    ->setRules([ '@Symfony' => true ])
    ->setFinder($finder);
