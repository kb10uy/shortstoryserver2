<?php

namespace App\Text\Parser;

use FFI;

class S3wf2Native
{
    /** @var FFI libs3wf2.so の FFI インスタンス */
    private FFI $library;

    public function __construct(string $libraryPath)
    {
        $this->library = FFI::load('
            struct environment_t;

            environment_t * s3wf2_init();
            int s3wf2_free(environment_t *);
            int s3wf2_parse(environment_t *, const char *);
            int s3wf2_get_document_string(environment_t *, char **);
            int s3wf2_get_document_buffered(environment_t *, int (*callback)(const char *, size_t, void *), void *);
            int s3wf2_free_string(char *);
            int s3wf2_get_next_error(environment_t *, char **);
            int s3wf2_reset_error(environment_t *);
        ', $libraryPath);
    }
};
