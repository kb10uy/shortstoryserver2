<?php

$finder = PhpCsFixer\Finder::create()
    ->exclude('./resources/views/**/*')
    ->exclude('./storage/**/*')
    ->exclude('./vendor/**/*')
    ->exclude('./_ide_helper.php')
    ->in(__DIR__);

return PhpCsFixer\Config::create()
    ->setRules([ '@Symfony' => true ])
    ->setFinder($finder);
