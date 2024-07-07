#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

#define DMSH_PROMPT "dmsh$ "
#define DMSH_BUFSIZE 1024
#define DMSH_ERRNEXIT(...) \
	do { \
		fprintf(stderr, __VA_ARGS__); \
		exit(EXIT_FAILURE); \
	} while(0)
#define DMSH_TOKEN_BUFSIZE 64
#define DMSH_TOKEN_DELIM " \t\r\n\a"
#define DMSH_PERRNEXIT(str) \
	do { \
		perror(str); \
		exit(EXIT_FAILURE); \
	} while(0)
#define DMSH_NO_STATUS 256 /* Maximum exit value for process is 255 */

int
dmsh_continue(const char *line)
{
	return strcmp(line, "");
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
dmsh_launch(char **args)
{
	pid_t pid, wpid;
	int status, ret;

	pid = fork();
	if (pid == 0) {
		/* I'm the child */
		if (execvp(args[0], args) < 0) {
			DMSH_PERRNEXIT("dmsh");
		}
	} else if (pid < 0) {
		DMSH_PERRNEXIT("dmsh");
	} else {
		/* I'm the parent and my child's ok */
		if ((wpid = wait(&status)) < 0) {
			DMSH_PERRNEXIT("dmsh");
		}
		if (WIFEXITED(status)) {
			/* Child called _exit, exit, or return from main */
			ret = WEXITSTATUS(status);
		} else {
			ret = DMSH_NO_STATUS;
		}
	}

	return ret;
}

int
dmsh_cd(char **args)
{
	if (args[1] == NULL) {
		fprintf(stderr, "dmsh: Must provide path to `cd` into\n");
		return 1;
	}
	if (chdir(args[1]) < 0) {
		perror("dmsh: cd");
		return 2;
	}

	return 0;
}

int
dmsh_exit(char **args)
{
	(void) args;
	exit(EXIT_SUCCESS);
	return 0;
}

char *dmsh_builtin_str[] = {
	"cd",
	"exit",
};

const int dmsh_num_builtins = sizeof(dmsh_builtin_str) / sizeof(char*);

int (*dmsh_builtin_func[])(char**) = {
	&dmsh_cd,
	&dmsh_exit,
};

int
dmsh_exec(char **args)
{
	int i;

	if (args[0] == NULL) {
		return 1;
	}
	for (i = 0; i < dmsh_num_builtins; i++) {
		if (!strcmp(dmsh_builtin_str[i], args[0])) {
			return dmsh_builtin_func[i](args);
		}
	}

	return dmsh_launch(args);
}

int
main(void)
{
	char *line;
	char **tokens;
	int ret = 0;

	printf("Type \"exit\" to exit the shell\n");
	do {
		printf("(%d) %s", ret, DMSH_PROMPT);
		line = dmsh_read_line();
		tokens = dmsh_split_line(line);
		ret = dmsh_exec(tokens); /* When we exit we don't free tokens. The OS
		                            does */
	} while (free(tokens), dmsh_continue_and_free(line));

	return EXIT_SUCCESS;
}
