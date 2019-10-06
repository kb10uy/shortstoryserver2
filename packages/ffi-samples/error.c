/*
 * Compile command (example):
 * gcc -std=c11 -L../s3wf2-rs/target/release -ls3wf2 test.c
 */

#include <stdio.h>
#include <stdlib.h>
#include "../s3wf2-rs/include/s3wf2.h"

const char *source =
    ":character male kb10uy kb10uy\n"
    ":character female natsuki 夏稀\n"
    "@kb10uy 「ついに僕たちも [m C 言語]に進出したよ」\n"
    "@mio 「いいハナシだねえ」\n"
    "@natsuki 「やったッスね先パイ！！」\n";

int main(void) {
    // create new parser
    s3wf2_parser *parser = s3wf2_parser_new();

    // parse the S3WF2 text...
    int status = s3wf2_parser_parse(parser, source);
    if (status == S3WF2_PARSE_ERROR) {
        char error[2048];

        printf("Parse Error!\n");
        while (s3wf2_parser_next_error(parser, error, 2047)) {
            printf("%s\n", error);
        }
    }

    // get the formatted HTML
    char *html = NULL;
    s3wf2_emit_html(parser, &html);

    // after use of returned HTML, we should free the buffer,
    // as it is allocated in library
    if (html) {
        puts(html);
        s3wf2_buffer_free(html);
    }

    // we should release the parser
    s3wf2_parser_free(parser);

    return 0;
}
