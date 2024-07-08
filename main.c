#include <assert.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
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
#define DMSH_QCAT "./qcat"
#define DMSH_LLS "./lls"

static int recvsig = 0; /* Have I received SIGINT? */
static int pressed_return = 0; /* User pressed return (instead of CTRL-D) */

int
dmsh_continue(const char *line)
{
	return strcmp(line, "") || recvsig || pressed_return;
}

int
dmsh_continue_and_handle(char *line)
{
	int ret = dmsh_continue(line);
	if (recvsig)
		printf("\n");
	recvsig = 0;
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
	for (;;) {
		c = getchar();
		if (c == EOF || c == '\n') {
			if (c == '\n')
				pressed_return = 1;
			else
				pressed_return = 0;
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
		if (execvp(args[0], args) < 0)
			DMSH_PERRNEXIT("dmsh");
	} else if (pid < 0) {
		DMSH_PERRNEXIT("dmsh");
	} else {
		/* I'm the parent and my child's ok */
		if ((wpid = wait(&status)) < 0)
			DMSH_PERRNEXIT("dmsh");
		if (WIFEXITED(status))
			/* Child called _exit, exit, or return from main */
			ret = WEXITSTATUS(status);
		else
			ret = DMSH_NO_STATUS;
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
	struct stat sb;
	char *new_args[] = {[1] = args[0], NULL};

	if (args[0] == NULL || !strcmp(args[0], ""))
		return !(recvsig || pressed_return);
	if (!args[1] && !access(args[0], R_OK)) {
		if (lstat(args[0], &sb) < 0)
			DMSH_PERRNEXIT("dmsh: lstat");
		switch (sb.st_mode & S_IFMT) {
		case S_IFREG:
			new_args[0] = DMSH_QCAT;
			return dmsh_launch(new_args);
		case S_IFDIR:
			new_args[0] = DMSH_LLS;
			return dmsh_launch(new_args);
		}
	}
	for (i = 0; i < dmsh_num_builtins; i++)
		if (!strcmp(dmsh_builtin_str[i], args[0]))
			return dmsh_builtin_func[i](args);

	return dmsh_launch(args);
}

static void
handler(int sig)
{
	(void) sig;
	recvsig = 1;
}

int
main(void)
{
	char *line;
	char **tokens;
	int ret = 0;
	struct sigaction sa;

	sa.sa_handler = handler;
	sigemptyset(&sa.sa_mask);
	sa.sa_flags = 0;
	if (sigaction(SIGINT, &sa, NULL) < 0)
		DMSH_PERRNEXIT("dmsh: sigaction");
	printf("Type \"exit\" to exit the shell\n");
	printf("You can also press CTRL-D on an empty line\n");
	printf("If you type a filename, dmsh will run `qcat` on it\n");
	printf("If you type a directory name, dmsh will run `lls` on it\n");
	do {
		printf("(%d) %s", ret, DMSH_PROMPT);
		line = dmsh_read_line();
		tokens = dmsh_split_line(line);
		ret = dmsh_exec(tokens); /* When we exit we don't free tokens. The OS
		                            does */
	} while (free(tokens), dmsh_continue_and_handle(line));
	printf("\n");

	return EXIT_SUCCESS;
}
