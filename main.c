#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define STR(x) #x
#define DMSH_TOKEN_EXIT STR(exit)
#define DMSH_PROMPT "dmsh$ " /* Using '%' instead of '$' needs escaping for
                                printf. */
#define DMSH_BUFSIZE 1024
#define DMSH_ERRNEXIT(...) \
	do { \
		fprintf(stderr, __VA_ARGS__); \
		exit(EXIT_FAILURE); \
	} while(0)
#define DMSH_TOKEN_BUFSIZE 64
#define DMSH_TOKEN_DELIM " \t\r\n\a"

int
dmsh_continue(const char *line)
{
	return strcmp(DMSH_TOKEN_EXIT, line) && strcmp(line, "");
}

int
dmsh_continue_and_free(char *line)
{
	int ret = dmsh_continue(line);
	free(line);
	return ret;
}

char *
dmsh_read_line(void)
{
	/* TODO Use getline
	 * See https://brennan.io/2015/01/16/write-a-shell-in-c/
	 * We do it like this since one of the purposes of this project is to
	 * learn.
	 */
	size_t bufsize = DMSH_BUFSIZE;
	size_t position = 0;
	char *buffer = malloc(bufsize * sizeof(char));
	int c;

	if (!buffer)
		DMSH_ERRNEXIT("dmsh: could not allocate buffer\n");
	for(;;) {
		c = getchar();
		if (c == EOF || c == '\n') {
			buffer[position] = '\0';
			return buffer;
		} else {
			buffer[position] = c;
		}
		position++; /* TODO Perhaps check overflow? */
		if (position >= bufsize) {
			bufsize += DMSH_BUFSIZE;
			buffer = realloc(buffer, bufsize);
			if (!buffer)
				DMSH_ERRNEXIT("dmsh: could not reallocate buffer\n");
		}
	}
}

char **
dmsh_split_line(char *line)
{
	size_t bufsize = DMSH_TOKEN_BUFSIZE, position = 0;
	char **tokens = malloc(bufsize * sizeof(char*));
	char *token;

	if(!tokens)
		DMSH_ERRNEXIT("dmsh: could not allocate token buffer\n");
	token = strtok(line, DMSH_TOKEN_DELIM); /* strtok is not thread safe but we
	                                           don't care */
	while(token) {
		tokens[position] = token;
		position++;
		if (position >= bufsize)  {
			tokens = realloc(tokens, bufsize * sizeof(char*));
			if (!tokens)
				DMSH_ERRNEXIT("dmsh: could not reallocate token buffer\n");
		}
		token = strtok(NULL, DMSH_TOKEN_DELIM);
	}
	tokens[position] = NULL;

	return tokens;
}

int
main(void)
{
	char *line;
	char **tokens;

	printf("Type \"exit\" to exit the shell\n");
	do {
		printf(DMSH_PROMPT);
		line = dmsh_read_line();
		tokens = dmsh_split_line(line);
	} while (free(tokens), dmsh_continue_and_free(line));

	return EXIT_SUCCESS;
}
