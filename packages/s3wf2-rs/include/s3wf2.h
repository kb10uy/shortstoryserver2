#ifndef S3WF2_H
#define S3WF2_H

struct s3wf2_parser_t;
typedef struct s3wf2_parser_t s3wf2_parser;

s3wf2_parser *s3wf2_parser_new(void);
void s3wf2_parser_free(s3wf2_parser *parser);

#endif
