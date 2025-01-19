#ifndef CLIQUE_H
#define CLIQUE_H
#include "main.h"

node_id_t* decode_clique(char* b64, node_id_t* length);
int test_clique(node_id_t* clique, node_id_t length, node_id_t** list, node_id_t nodes);

#endif