<?php

namespace App\Text\Parser;

use FFI;

class S3wf2Native
{
    public const STATUS_SUCCESS = 0;
    public const STATUS_NO_MORE_ERROR = 1;
    public const STATUS_ERROR = 2;
    public const STATUS_INVALID_ENVIRONMENT = 3;
    public const STATUS_INVALID_SOURCE = 4;
    public const STATUS_PARSE_ERROR = 5;

    /**
     * @var mixed libs3wf2.so の FFI インスタンス。
     * 本当は `FFI` なんだけど PHPStan の黙らせ方がわからないので……
     */
    private $library;

    public function __construct(string $libraryPath)
    {
        $this->library = FFI::cdef('
            struct environment_t;

            struct environment_t * s3wf2_init();
            int s3wf2_free(struct environment_t *);
            int s3wf2_parse(struct environment_t *, const char *);
            int s3wf2_get_document_string(struct environment_t *, char **);
            int s3wf2_get_document_buffered(struct environment_t *, int (*callback)(const char *, size_t, void *), void *);
            int s3wf2_free_string(char *);
            int s3wf2_get_next_error(struct environment_t *, char **);
            int s3wf2_reset_error(struct environment_t *);
        ', $libraryPath);
    }

    /**
     * libs3wf2 を利用して HTML を生成する。
     */
    public function generateHtml(string $source): string
    {
        $environment = $this->library->s3wf2_init();
        $status = $this->library->s3wf2_parse($environment, $source);

        $html = '';
        switch ($status) {
            case self::STATUS_SUCCESS:
                $documentPointer = FFI::new('char *');
                $this->library->s3wf2_get_document_string($environment, FFI::addr($documentPointer));
                $html = FFI::string($documentPointer);
                $this->library->s3wf2_free_string($documentPointer);
                break;

            case self::STATUS_PARSE_ERROR:
                $html = "<ul>\n";
                $errorPointer = FFI::new('char *');
                while (self::STATUS_NO_MORE_ERROR != $this->library->s3wf2_get_next_error($environment, FFI::addr($errorPointer))) {
                    $html .= '<li>' . htmlspecialchars(FFI::string($errorPointer)) . "</li>\n";
                    $this->library->s3wf2_free_string($errorPointer);
                }
                $html .= '</ul>';
                break;

            default:
                $html = 'Internal parser error occured. Please contact to kb10uy.';
                break;
        }

        $this->library->s3wf2_free($environment);

        return $html;
    }
}
