#ifndef ADJ_LIST_H
#define ADJ_LIST_H
#include "main.h"

int sigma(char* array, int n);
int in(node_id_t* array, node_id_t e, int length);
int empty(node_id_t** list, node_id_t nodes);
node_id_t** decode_list(char* b64, node_id_t* nodes);
void free_list(node_id_t** list, node_id_t nodes);
#ifdef DEBUG
void print_list(node_id_t** list, node_id_t nodes);
#endif

#endif