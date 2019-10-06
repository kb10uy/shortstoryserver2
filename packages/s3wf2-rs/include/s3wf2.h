#ifndef S3WF2_H
#define S3WF2_H

#include <stddef.h>

struct s3wf2_parser_t;

enum s3wf2_status_t {
    S3WF2_SUCCESS = 0,
    S3WF2_INVALID,
    S3WF2_INVALID_SOURCE,
    S3WF2_PARSE_ERROR,
};

/**
 * Represents status of S3WF2 parser.
 */
typedef enum s3wf2_status_t s3wf2_status;

/**
 * An opaque struct of S3WF2 parser.
 */
typedef struct s3wf2_parser_t s3wf2_parser;

s3wf2_parser *s3wf2_parser_new(void);
void s3wf2_parser_free(s3wf2_parser *parser);
s3wf2_status s3wf2_parser_reset(s3wf2_parser *parser);
s3wf2_status s3wf2_parser_parse(s3wf2_parser *parser, const char *source);
int s3wf2_parser_next_error(s3wf2_parser *parser, char *buffer, size_t buffer_length);
s3wf2_status s3wf2_emit_html(const s3wf2_parser *parser, char **buffer);

#endif
