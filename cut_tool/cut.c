#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct Option {
    int *fields;
    char delimeter;
} Option;

void parse_commands(int argc, char **argv, char *filenames[], Option *option);
void read_from_files(char **filenames, int len, char *delimeter, Option option);
void print_fatal_error(char *msg);
void procces_file(FILE *file, char *delimeter, Option option);

int main(int argc, char **argv) {
    char *filenames[argc];
    Option option = {.delimeter = '\t', .fields = NULL};
    parse_commands(argc, argv, filenames, &option);

    if (option.fields == NULL) {
        print_fatal_error("0 fields specified");
    }

    char delimeter[4] = {0};
    sprintf(delimeter, "%c", option.delimeter);

    if (filenames[0] == NULL) {
        procces_file(stdin, delimeter, option);

    } else {
        read_from_files(filenames, argc, delimeter, option);
    }
}

void read_from_files(char **filenames, int len, char *delimeter,
                     Option option) {

    for (int i = 0; i < len; i++) {
        char *fln = filenames[i];

        if (fln == NULL)
            break;

        FILE *file = fopen(fln, "rb");

        if (file == NULL)
            print_fatal_error("File does not exist");

        procces_file(file, delimeter, option);
        fclose(file);
    }
}

void procces_file(FILE *file, char *delimeter, Option option) {
    char line[256] = {0};

    while ((fgets(line, sizeof(line), file)) != NULL) {
        char *token = strtok(line, delimeter);
        int count = 1;

        while (token != NULL) {
            for (int i = 0;; i++) {
                int value = option.fields[i];
                if (value == -1)
                    break;

                if (value == count) {
                    printf("%s%s", token, delimeter);
                }
            }

            token = strtok(NULL, delimeter);
            count++;
        }
        printf("\n");
    }
}

void parse_commands(int argc, char **argv, char *filenames[], Option *option) {
    int lenfl = 0;

    for (int i = 1; i < argc; i++) {
        char *arg = argv[i];
        if (*arg == '-') {
            char ch = *(arg + 1);
            if (ch == 'f') {
                char *rest = strdup(arg + 2);
                int fc = 0;
                char *token = strtok(rest, " , ");
                while (token) {
                    token = strtok(NULL, " , ");
                    fc++;
                }

                free(rest);

                if (fc <= 0) {
                    print_fatal_error(" 0 fields found");
                }

                int *nf = malloc(sizeof(int) * (fc + 1));
                char *tkn = strtok(arg + 2, " , ");
                int lf = 0;

                while (tkn) {
                    int v;
                    int res = sscanf(tkn, "%d", &v);
                    if (res != 1) {
                        print_fatal_error("Option field is not valid");
                    }
                    nf[lf++] = v;
                    tkn = strtok(NULL, " , ");
                }
                nf[lf] = -1;
                option->fields = nf;

            } else if (ch == 'd') {
                char delimeter = *(arg + 2);
                if (delimeter == '\0') {
                    print_fatal_error("wrong delimeter");
                }
                option->delimeter = delimeter;

            } else {
                print_fatal_error("Invalid option");
            }
        } else {
            filenames[lenfl++] = arg;
        }
    }

    filenames[lenfl] = NULL;
}

void print_fatal_error(char *msg) {
    fprintf(stderr, "%s", msg);
    exit(EXIT_FAILURE);
}
