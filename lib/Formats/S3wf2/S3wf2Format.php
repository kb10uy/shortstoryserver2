<?php

declare(strict_types=1);

namespace Lib\Formats\S3wf2;

use Illuminate\Support\Collection;
use Lib\Formats\Format;
use Lib\Formats\Node;
use Lib\Formats\SingularNode;
use Lib\Formats\ParseErrorException;

/**
 * ShortStoryServer Writer's Format v2.
 */
class S3wf2Format extends Format
{
    /** @var Collection */
    private $allowedBlocks;

    /** @var Collection */
    private $allowedNonClosingBlocks;

    /** @var Collection */
    private $allowedPhrasings;

    /** @var CharacterSet */
    private $characters;

    /** @var Node */
    private $rootNode;

    /** @var Node */
    private $currentParagraph;

    /** @var bool */
    private $paragraphInUse;

    /** @var Node */
    private $errors;

    /** @var bool */
    private $errorOccurred;

    public function __construct()
    {
        $this->reset();
        $this->registerTags();
    }

    /**
     * 状態をリセットする。
     */
    public function reset(): void
    {
        $this->characters = new CharacterSet();
        $this->rootNode = new Node('article', collect(['class' => 'post']));
        $this->rootNode->addTextNode(PHP_EOL);
        $this->currentParagraph = new Node('p');
        $this->currentParagraph->addTextNode(PHP_EOL);
        $this->paragraphInUse = false;
        $this->errors = new Node('ul', collect(['class' => 's3wf2-errors']));
        $this->errors->addTextNode(PHP_EOL);
        $this->errorOccurred = false;
    }

    /**
     * ソースを読み込んでパースする。
     *
     * @param string $source ソースコード
     */
    public function parse(string $source): void
    {
        $sourceLines = preg_split('/\r\n|\r|\n/', $source) ?: [];
        foreach ($sourceLines as $lineNumber => $lineString) {
            ++$lineNumber;
            $lineString = trim($lineString);
            if (0 === strpos($lineString, '//')) {
                continue;
            }

            if ('' === $lineString) {
                $this->commitParagraph();
                continue;
            }
            try {
                if (1 === preg_match('/^(:|\/|@)(\w+)(\s+(.*))?$/u', $lineString, $matches)) {
                    // escaped line
                    switch ($matches[1]) {
                        case ':':
                            $command = $matches[2];
                            $rawParams = preg_split('/\s+/', $matches[4] ?? '', -1, PREG_SPLIT_NO_EMPTY) ?: [];
                            $this->processSourceCommandLine($command, collect($rawParams));
                            break;
                        case '/':
                            $tag = $matches[2];
                            $line = $matches[4] ?? '';
                            $this->processSourceBlockLine($tag, $line);
                            break;
                        case '@':
                            $characterKey = $matches[2];
                            $line = $matches[4] ?? '';
                            $this->processSourceSpeechLine($characterKey, $line);
                            break;
                    }
                } else {
                    $this->processSourceNormalLine($this->currentParagraph, $lineString);
                    $this->currentParagraph->addTextNode(PHP_EOL);
                }
            } catch (ParseErrorException $ex) {
                $errorNode = new Node('li');
                $errorNode->addTextNode("[Line $lineNumber] {$ex->getMessage()}");
                $this->errors->addNode($errorNode);
                $this->errors->addTextNode(PHP_EOL);
                $this->errorOccurred = true;
            }
        }
        $this->commitParagraph();
    }

    /**
     * HTML を生成する。
     *
     * @return string 生成されるHTML
     */
    public function toHtml(): string
    {
        ob_start();
        echo $this->characters->generateCustomColorsStyle()->emit();
        echo PHP_EOL;

        if ($this->errorOccurred) {
            echo $this->errors->emit();
            echo PHP_EOL;
        }

        echo $this->rootNode->emit();
        echo PHP_EOL;

        return ob_get_clean() ?: '';
    }

    /**
     * コマンド行の処理.
     *
     * @param string     $command コマンド名
     * @param Collection $params  パラメーター
     */
    private function processSourceCommandLine(string $command, Collection $params): void
    {
        switch ($command) {
            case 'character':
                // :character <type> <key> <name>
                $this->characters->set($params[1], $params[0], $params[2]);
                break;
            default:
                throw new ParseErrorException("Unknown command: $command");
                break;
        }
    }

    /**
     * 現在の段落を確定する。
     */
    private function commitParagraph(): void
    {
        if (!$this->paragraphInUse) {
            return;
        }

        $this->rootNode->addNode($this->currentParagraph);
        $this->rootNode->addTextNode(PHP_EOL);
        $this->currentParagraph = new Node('p');
        $this->currentParagraph->addTextNode(PHP_EOL);
        $this->paragraphInUse = false;
    }

    /**
     * ソースの中のブロック要素の行を処理する。
     *
     * @param string $tag  タグ名
     * @param string $line 行。 >>> なら複数行開始、 <<< なら複数行終了
     */
    private function processSourceBlockLine(string $tag, string $line): void
    {
        // 閉じタグできないやつは優先的に対応
        $ncel = $this->allowedNonClosingBlocks->get($tag);
        if ($ncel) {
            $this->commitParagraph();
            $node = new SingularNode($ncel[0]);
            $this->currentParagraph = $node;
            $this->paragraphInUse = true;
            $this->commitParagraph();

            return;
        }

        $el = $this->allowedBlocks->get($tag);
        if (!$el) {
            throw new ParseErrorException("Unknown inline tag: $tag");
        }

        switch ($line) {
            case '>>>':
                $this->commitParagraph();
                $node = new Node($el[0], collect(['class' => $el[1]]));
                $this->currentParagraph = $node;
                $this->currentParagraph->addTextNode(PHP_EOL);
                break;
            case '<<<':
                $this->commitParagraph();
                break;
            default:
                $this->commitParagraph();
                $node = new Node($el[0], collect(['class' => $el[1]]));
                $this->currentParagraph = $node;
                $this->processSourceNormalLine($this->currentParagraph, $line);
                $this->commitParagraph();
                break;
        }
    }

    /**
     * ソースの中の通常の行やインライン部分を処理する。
     *
     * @param string $characterKey キャラクターの参照名
     * @param string $line         行
     */
    private function processSourceSpeechLine(string $characterKey, string $line): void
    {
        $character = $this->characters->get($characterKey);
        if (!$character) {
            throw new ParseErrorException("Unknown character: $characterKey");
        }

        $name = $character->displayName();
        $color = $character->colorClass();

        $attributes = collect(['class' => "line $color"]);
        $node = new Node('span', $attributes);
        $node->addTextNode($name);

        $this->processSourceNormalLine($node, $line);
        $this->currentParagraph->addNode($node);
        $this->currentParagraph->addTextNode(PHP_EOL);
        $this->paragraphInUse = true;
    }

    /**
     * ソースの中の通常の行やインライン部分を処理する。
     *
     * @param Node   $parent 親ノード
     * @param string $line   行
     */
    private function processSourceNormalLine(Node $parent, string $line): void
    {
        $stack = collect([$parent]);
        $rest = $line;

        while ('' !== $rest) {
            $tagFound = preg_match('/\[(@?(\w+))(?:\s+|(?=[{\[]))|[{}\]]|\[(\w+)\]/u', $rest, $matches, PREG_OFFSET_CAPTURE);

            // ケツまでタグなし
            if (1 !== $tagFound) {
                $stack->last()->addTextNode($rest);
                $rest = '';
                continue;
            }

            // タグ区切りまでは今のトップに入れる
            $tagString = $matches[0][0];
            $tagPosition = $matches[0][1];
            $before = substr($rest, 0, $tagPosition);
            $stack->last()->addTextNode($before);

            if ('{' === $tagString) {
                // パラメーター開き
                if (1 === $stack->count()) {
                    throw new ParseErrorException('No parent tag found');
                }
                $parameter = new Node('*');
                $parameter->setEmitter(function ($tag, $attrs, $params, $nodes) {
                    ob_start();
                    foreach ($nodes as $node) {
                        echo (string) $node;
                    }

                    return ob_get_clean() ?: '';
                });
                $stack->push($parameter);
            } elseif ('}' === $tagString) {
                // パラメーター閉じ
                if (1 === $stack->count()) {
                    throw new ParseErrorException('No parameter there');
                }
                $parameter = $stack->pop();
                if ('*' !== $parameter->tagName()) {
                    throw new ParseErrorException('Invalid parameter');
                }
                $stack->last()->addParameter($parameter);
            } elseif (']' === $tagString) {
                // タグ閉じ
                if (1 === $stack->count()) {
                    // 閉じタグが多すぎる
                    throw new ParseErrorException('Too many closing tag: ' . $line);
                }
                $node = $stack->pop();
                $stack->last()->addNode($node);
            } elseif ('[' === $tagString[0] && ']' === substr($tagString, -1)) {
                // 単一タグ
                if ('[br]' !== $tagString) {
                    // TODO: そのうち <br> 以外にも対応する
                    throw new ParseErrorException('Other than [br] is unacceptable');
                }
                $stack->last()->addNode(new SingularNode('br'));
            } else {
                // タグ開き
                $tagName = $matches[1][0];
                if ('@' === $tagName[0]) {
                    // インライン台詞
                    $character = $this->characters->get($matches[2][0]);
                    if (!$character) {
                        throw new ParseErrorException("Unknown character: {$matches[2]}");
                    }
                    $attributes = collect(['class' => "line inline {$character->colorClass()}"]);
                    $node = new Node('span', $attributes);
                } else {
                    $el = $this->allowedPhrasings->get($tagName);
                    if (!$el) {
                        throw new ParseErrorException("Unknown inline tag: $tagName");
                    }

                    $node = new Node($el[0], collect(['class' => $el[1]]));
                    if (isset($el[2])) {
                        $node->setEmitter($el[2]);
                    }
                }
                $stack->push($node);
            }
            $rest = substr($rest, (int) $tagPosition + strlen($tagString));
        }

        if (1 !== $stack->count()) {
            // 閉じタグが少なすぎる
            throw new ParseErrorException('Too many closing tag: ' . $line);
        }

        $this->paragraphInUse = true;
    }

    private function registerTags(): void
    {
        // 要素は [タグ, クラス]

        $this->allowedNonClosingBlocks = collect();
        $this->allowedNonClosingBlocks['hori'] = ['hr', ''];

        $this->allowedBlocks = collect();
        $this->allowedBlocks['para'] = ['p', ''];
        $this->allowedBlocks['quote'] = ['blockquote', ''];
        $this->allowedBlocks['sec'] = ['h2', 'section'];
        $this->allowedBlocks['subsec'] = ['h3', 'section'];
        $this->allowedBlocks['enum'] = ['il', ''];
        $this->allowedBlocks['list'] = ['ul', ''];

        $this->allowedPhrasings = collect();
        $this->allowedPhrasings['item'] = ['li', ''];
        $this->allowedPhrasings['b'] = ['strong', ''];
        $this->allowedPhrasings['i'] = ['i', ''];
        $this->allowedPhrasings['m'] = ['code', ''];
        $this->allowedPhrasings['ul'] = ['span', 'underline'];
        $this->allowedPhrasings['st'] = ['del', ''];
        $this->allowedPhrasings['dt'] = ['span', 'dots'];

        $this->allowedPhrasings['link'] = ['a', '', function ($tag, $attrs, $params, $nodes) {
            $target = $params->count() >= 1 ? htmlspecialchars($params[0]->emitPlain()) : '#';
            ob_start();
            echo "<a href=\"$target\">";
            foreach ($nodes as $node) {
                echo (string) $node;
            }
            echo '</a>';

            return ob_get_clean() ?: '';
        }];

        $this->allowedPhrasings['ruby'] = ['ruby', '', function ($tag, $attrs, $params, $nodes) {
            $rubyHtml = $params->count() >= 1 ? $params[0]->emit() : '';

            ob_start();
            echo '<ruby>';
            foreach ($nodes as $node) {
                echo (string) $node;
            }
            echo '<rp>(</rp><rt>';
            echo $rubyHtml;
            echo '</rt><rp>)</rp>';
            echo '</ruby>';

            return ob_get_clean() ?: '';
        }];
    }
}
