#include "clique.h"
#include "adj_list.h"
#include "main.h"
#include "base64.h"
#include <stdio.h>

node_id_t* decode_clique(char* b64, node_id_t* length) {
	int declen;
	char* dec = base64_decode(b64, &declen);
	// testy poprawności
	if(declen < 3) {
		free(dec);
		return NULL;
	}
	*length = ((node_id_t)(dec[1]) << 8) + ((node_id_t)(dec[0]));
	if((declen - 2) != (*length * 2)) {
		free(dec);
		return NULL;
	}

	node_id_t* clique = malloc(*length * sizeof(node_id_t));
	int pos = 2;
	for(int i = 0; i < *length; i++) {
		clique[i] = ((node_id_t)(dec[pos + 1]) << 8) + ((node_id_t)(dec[pos]));
		pos += 2;
	}
	free(dec);
	return clique;
}

int test_clique(node_id_t* clique, node_id_t length, node_id_t** list, node_id_t nodes) {
	for(int i = 0; i < (length - 1); i++) {
		if(list[clique[i]] == NULL)
			return 0;
		for(int j = (i + 1); j < length; j++) {
			#ifdef DEBUG
			printf(" - %i-%i\n", clique[i], clique[j]);
			#endif
			// czy klika rzeczywiście jest kliką
			if(!in(list[clique[i]] + 1, clique[j], list[clique[i]][0]))
				return 0;
		}
	}
	for(int i = 0; i < length; i++) {
		free(list[clique[i]]);
		list[clique[i]] = NULL;
	}
	return 1;
}