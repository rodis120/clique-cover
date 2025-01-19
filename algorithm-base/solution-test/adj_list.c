#include "adj_list.h"
#include "base64.h"
#include "main.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int sigma(char* array, int n) {
	int sum = 0;
	for(int i = 0; i < n; i++)
		sum += array[i];
	return sum;
}

int in(node_id_t* array, node_id_t e, int length) {
	for(int i = 0; i < length; i++) {
		if(array[i] == e)
			return 1;
	}
	return 0;
}

int empty(node_id_t** list, node_id_t nodes) {
	for(int i = 0; i < nodes; i++) {
		if(list[i])
			return 0;
	}
	return 1;
}

node_id_t** decode_list(char* b64, node_id_t* nodes) {
	int len;
	char* str = base64_decode(b64, &len);

	// testy poprawności
	if(len < 3) {
		free(str);
		return NULL;
	}
	*nodes = ((node_id_t)(str[1]) << 8) + ((node_id_t)(str[0]));
	if((len - 2) != ((((*nodes * (*nodes + 1)) / 2) + 7) / 8)) {
		free(str);
		return NULL;
	}

	node_id_t** list = calloc(*nodes, sizeof(node_id_t*));
	int byte = 2;
	int bit = 1;
	char* row = malloc(*nodes * sizeof(char));
	for(int i = 0; i < *nodes; i++) {
		memset(row, 0, *nodes * sizeof(char));
		for(int j = i; j < *nodes; j++) {
			if(str[byte] & bit) {
				row[j] = 1;
			}
			bit <<= 1;
			if(bit == 0x100) { // jedynka na dziewiątym bicie
				byte++;
				bit = 1;
			}
		}
		int n = sigma(row, *nodes);
		if(n == 0)
			continue;
		list[i] = malloc((n + 1) * sizeof(node_id_t));
		list[i][0] = n; // długość listy następników zapisana na pozycji 0
		// konstrukcja listy następników
		int ii = 1;
		for(int j = 0; j < *nodes; j++) {
			if(row[j])
				list[i][ii++] = j;
			if(ii == (n + 1))
				break;
		}
	}
	free(row);
	free(str);
	return list;
}

void free_list(node_id_t** list, node_id_t nodes) {
	for(int i = 0; i < nodes; i++) {
		if(list[i])
			free(list[i]);
	}
	free(list);
}

#ifdef DEBUG
void print_list(node_id_t** list, node_id_t nodes) {
	for(int i = 0; i < nodes; i++) {
		printf("%i: ", i);
		if(list[i]) {
			for(int j = 1; (j < (list[i][0]) + 1); j++) {
				printf("%i ", list[i][j]);
			}
			puts("");
		}
		else
			puts("∅");
	}
}
#endif