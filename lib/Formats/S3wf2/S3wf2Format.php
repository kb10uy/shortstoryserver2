<?php

declare(strict_types=1);

namespace Lib\Formats\S3wf2;

use Lib\Formats\Format;
use Lib\Formats\Node;
use Lib\Formats\ParseErrorException;

class S3wf2Format extends Format
{
    private $characters;
    private $rootNode;
    private $currentParagraph;
    private $paragraphInUse;

    public function __construct()
    {
        $this->reset();
    }

    /**
     * 状態をリセットする。
     */
    public function reset(): void
    {
        $this->characters = collect();
        $this->rootNode = new Node('article');
        $this->rootNode->addTextNode(PHP_EOL);
        $this->currentParagraph = new Node('p');
        $this->currentParagraph->addTextNode(PHP_EOL);
        $this->paragraphInUse = false;
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
            $lineString = trim($lineString);
            if (0 === strpos($lineString, '//')) {
                continue;
            }

            if (1 === preg_match('/^(:|\/|@)(\w+)\s+(.+)$/u', $lineString, $matches)) {
                // escaped line
                switch ($matches[1]) {
                    case ':':
                        break;
                    case '/':
                        $tag = $matches[2];
                        $line = $matches[3];
                        $this->processSourceBlockLine($tag, $line);
                        break;
                    case '@':
                        $characterKey = $matches[2];
                        $line = $matches[3];
                        $this->processSourceSpeechLine($characterKey, $line);
                        break;
                }
            } else {
                $this->processSourceNormalLine($this->currentParagraph, $lineString);
                $this->currentParagraph->addTextNode(PHP_EOL);
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
        return $this->rootNode->emit();
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
        switch ($line) {
            case '>>>':
                $this->commitParagraph();
                $node = new Node($tag);
                $this->currentParagraph = $node;
                $this->currentParagraph->addTextNode(PHP_EOL);
                break;
            case '<<<':
                $this->commitParagraph();
                break;
            default:
                $this->commitParagraph();
                $node = new Node($tag);
                $this->currentParagraph = $node;
                $this->processSourceNormalLine($this->currentParagraph, $line);
                $this->commitParagraph();
                break;
        }
    }

    /**
     * ソースの中の通常の行やインライン部分を処理する。
     *
     * @param string  $characterKey キャラクターの参照名
     * @param strinng $line         行
     */
    private function processSourceSpeechLine(string $characterKey, string $line): void
    {
        $attributes = collect(['class' => 'line']);
        $node = new Node('span', $attributes);
        $node->addTextNode('');
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
            $tagFound = preg_match('/\[(\w+)\s+|\]/u', $rest, $matches, PREG_OFFSET_CAPTURE);

            // ケツまでタグなし
            if ($tagFound !== 1) {
                $stack->last()->addTextNode($rest);
                $rest = '';
                continue;
            }

            // タグ区切りまでは今のトップに入れる
            $tagString = $matches[0][0];
            $tagPosition = $matches[0][1];
            $before = substr($rest, 0, $tagPosition);
            $stack->last()->addTextNode($before);

            if (']' === $tagString) {
                if (1 === $stack->count()) {
                    // 閉じタグが多すぎる
                    throw new ParseErrorException('Too many closing tag: ' . $line);
                }
                $node = $stack->pop();
                $stack->last()->addNode($node);
            } else {
                $tagName = $matches[1][0];
                $node = new Node($tagName);
                $stack->push($node);
            }
            $rest = substr($rest, $tagPosition + strlen($tagString));
        }

        if (1 !== $stack->count()) {
            // 閉じタグが少なすぎる
            throw new ParseErrorException('Too many closing tag: ' . $line);
        }

        $this->paragraphInUse = true;
    }
}
