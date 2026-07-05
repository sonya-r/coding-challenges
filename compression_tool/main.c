// WELL IT WORKS LOL
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_LEN 255

typedef struct Bits {
    unsigned int depth;
    unsigned char container[100];
} Bits;

typedef struct HuffNode {
    int key;
    int weight;
    struct HuffNode *right;
    struct HuffNode *left;
    struct Bits code;
} HuffNode;

Bits shift_bits(Bits bits, unsigned char val);
void clear_tree(HuffNode *root);
HuffNode *build_hufftree(const int *array);
void fill_codes(HuffNode *node, Bits code);
void store_in_array(HuffNode *array[], HuffNode *node);
void write_encoded(FILE *src, FILE *dest, HuffNode **map);
void write_headers(FILE *dest, HuffNode **huffarray);
void decoder(char *filename);
void encoder(char *filename);

int main(int argc, char *argv[]) {

    if (argc <= 1) {
        fprintf(stderr, "Filename is empty\n");
        exit(EXIT_FAILURE);
    }

    if (strstr(argv[1], ".huff") == NULL) {
        encoder(argv[1]);
    } else {
        decoder(argv[1]);
    }

    return EXIT_SUCCESS;
}
FILE *openfile(char *filename, char *mode) {
    FILE *file;
    int chr;

    file = fopen(filename, mode);

    if (file == NULL) {
        fprintf(stderr, "File does not exist: %s\n", filename);
        exit(EXIT_FAILURE);
    }

    return file;
}

void insert_bit(Bits *bits, unsigned char val) {
    bits->container[bits->depth / 8] =
        (bits->container[bits->depth / 8] << 1) | val;

    bits->depth++;
}

bool compare_bits(Bits a, Bits b) {
    if (a.depth != b.depth)
        return false;

    for (int i = 0; i <= ((a.depth - 1) / 8); i++) {
        if (a.container[i] != b.container[i])
            return false;
    }

    return true;
}

void decoder(char *filename) {
    HuffNode *array[MAX_LEN] = {0};
    int i = 0;
    FILE *file = openfile(filename, "rb");

    while (true) {
        HuffNode node;
        fread(&node, sizeof(struct HuffNode), 1, file);

        if (node.key == '\0')
            break;

        HuffNode *e = malloc(sizeof(struct HuffNode));
        memcpy(e, &node, sizeof(struct HuffNode));
        array[i++] = e;
    };

    Bits current = {0};
    int ch;

    FILE *dest = openfile("decoded", "wb");

    while ((ch = fgetc(file)) != EOF) {
        unsigned char byte = ch;

        for (int i = 0; i < 8; i++) {
            insert_bit(&current, (byte & 0x80) == 0x80 ? 1 : 0);
            byte <<= 1;

            for (int j = 0; j < MAX_LEN; j++) {
                HuffNode *node = array[j];

                if (node == NULL) {
                    break;
                }

                if (compare_bits(current, node->code)) {
                    fputc(node->key, dest);
                    Bits x = {0};
                    current = x;
                    break;
                }
            }
        }
    }

    for (int j = 0; j < MAX_LEN; j++) {
        if (array[j] == NULL)
            continue;

        free(array[j]);
    }

    fclose(file);
}

void encoder(char *filename) {
    int characters[MAX_LEN] = {0};
    FILE *src = openfile(filename, "rb");
    int chr;

    while ((chr = fgetc(src)) != EOF) {
        characters[chr] += 1;
    }

    HuffNode *root = build_hufftree(characters);
    fill_codes(root, (Bits){0});
    HuffNode *huffarray[MAX_LEN] = {0};
    store_in_array(huffarray, root);
    char dest_filename[strlen(filename) + 10];
    strcpy(dest_filename, filename);
    strcat(dest_filename, ".huff");
    FILE *dest = openfile(dest_filename, "wb");
    fseek(src, 0, SEEK_SET);
    write_headers(dest, huffarray);
    write_encoded(src, dest, huffarray);

    clear_tree(root);
    fclose(src);
    fclose(dest);
}

void clear_tree(HuffNode *root) {

    if (root == NULL)
        return;

    clear_tree(root->right);
    clear_tree(root->left);

    free(root);
}

void write_headers(FILE *dest, HuffNode **huffarray) {

    for (int i = 0; i < MAX_LEN; i++) {
        if (huffarray[i] == NULL)
            continue;
        fwrite(huffarray[i], sizeof(struct HuffNode), 1, dest);
    }

    HuffNode end = {.key = '\0'};
    fwrite(&end, sizeof(struct HuffNode), 1, dest);
}

void insert_last_bit(Bits *dest, unsigned char *src, int index, int depth) {
    unsigned char bit;

    if (index / 8 == depth / 8) {
        bit = (*src >> (depth % 8 - (index + 1) % 8));
    } else {
        bit = (*src >> (8 - ((index) % 8 + 1)));
    }

    insert_bit(dest, bit & 1);
}

void write_encoded(FILE *src, FILE *dest, HuffNode **map) {
    Bits tw = {0};
    int ch;

    while ((ch = fgetc(src)) != EOF) {
        HuffNode *node = map[ch];

        if (node == NULL) {
            fprintf(stderr, "Key was not found???????");
            exit(EXIT_FAILURE);
        }

        for (int i = 0; i < node->code.depth; i++) {
            unsigned char *srcbit = &node->code.container[i / 8];
            insert_last_bit(&tw, srcbit, i, node->code.depth);

            if (tw.depth == 8) {
                fwrite(&tw.container[0], sizeof(unsigned char), 1, dest);
                tw.depth = 0;
                tw.container[0] = 0;
            }
        }
    }
}

void store_in_array(HuffNode *array[], HuffNode *node) {
    if (node == NULL)
        return;

    if (node->key != -1) {
        array[node->key] = node;
    }

    store_in_array(array, node->left);
    store_in_array(array, node->right);
}

int compare_function(const void *_a, const void *_b) {
    const HuffNode *a = *((HuffNode **)_a);
    const HuffNode *b = *((HuffNode **)_b);
    return b->weight - a->weight;
}

HuffNode *build_hufftree(const int *array) {
    HuffNode *huffarray[MAX_LEN] = {0};
    int top = 0;

    for (int ch = 0; ch < MAX_LEN; ch++) {
        int frequency = array[ch];

        if (frequency > 0) {
            HuffNode *huffnode = malloc(sizeof(struct HuffNode));

            huffnode->key = ch;
            huffnode->weight = frequency;
            huffnode->left = NULL;
            huffnode->right = NULL;
            huffarray[top++] = huffnode;
        }
    }

    while (top > 1) {
        qsort(huffarray, top, sizeof(struct HuffNode *), compare_function);

        HuffNode *a = huffarray[--top];
        HuffNode *b = huffarray[--top];
        HuffNode *c = malloc(sizeof(struct HuffNode));

        c->weight = a->weight + b->weight;
        c->left = a;
        c->right = b;
        c->key = -1;

        huffarray[top++] = c;
    }

    HuffNode *root = huffarray[0];

    return root;
}

Bits shift_bits(Bits bits, unsigned char val) {
    insert_bit(&bits, val);
    return bits;
}

void fill_codes(HuffNode *node, Bits code) {
    if (node == NULL)
        return;

    node->code = code;

    fill_codes(node->left, shift_bits(code, 0));
    fill_codes(node->right, shift_bits(code, 1));
}
