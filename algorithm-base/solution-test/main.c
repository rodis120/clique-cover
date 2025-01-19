#include "clique.h"
#include "adj_list.h"
#include "main.h"
#include "base64.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define BUFLEN 1024

char* encode_number(node_id_t n) {
	char n_char[sizeof(node_id_t) + 1];
	for(int i = 0; i < sizeof(node_id_t); i++) {
		n_char[i] = n & 0xff;
		n >>= 8;
	}
	n_char[sizeof(node_id_t)] = 0;
	return base64_encode(n_char);
}

void insert(char*** array, int index, int* size, char* e) {
	(*array)[index] = e;
	if((index + 1) > (*size - 1)) { // może trzeba zwiększyć argv
		*size *= 2;
		*array = realloc(*array, *size * sizeof(char*));
	}
}

void stdin_args(int* argc, char*** argv) {
	*argc = 1;
	int argv_size = 4;
	int s_size = BUFLEN;
	int s_pos = 0;
	{
		char** argv_new = malloc(argv_size * sizeof(char*)); // dynamiczna tablica
		memcpy(argv_new, *argv, *argc * sizeof(char*));
		*argv = argv_new;
	}
	char* s = malloc(BUFLEN * sizeof(char));
	int c = 0;
	while((c = fgetc(stdin)) != EOF) {
		s[s_pos] = c;
		if(isspace(c) || (c < 32)) {
			if(s_pos > 0) {
				s[s_pos] = 0;
				s = realloc(s, (s_pos + 1) * sizeof(char));
				insert(argv, (*argc)++, &argv_size, s);

				s_size = BUFLEN;
				s = malloc(BUFLEN * sizeof(char));
			}
			s_pos = 0;
			continue;
		}
		else if(s_pos == (s_size - 1)) {
			s_size += BUFLEN;
			s = realloc(s, s_size * sizeof(char));
		}
		s_pos++;
	}
	if(s_pos > 0) {
		s[s_pos] = 0;
		s = realloc(s, (s_pos + 1) * sizeof(char));
		insert(argv, (*argc)++, &argv_size, s);
	}
}

int main(int argc, char** argv) {
	if(argc == 1) { // tryb stdin (zamiast argumentów z konsoli)
		stdin_args(&argc, &argv);
	}
	
	if(argc < 3) {
		fputs("Zbyt mało argumentów\n", stderr);
		return 1;
	}

	node_id_t nodes;
	node_id_t** list = decode_list(argv[1], &nodes);
	if(list == NULL) {
		fputs("Błąd odczytu grafu\n", stderr);
		return 1;
	}

	#ifdef DEBUG
	printf("Nodes: %i\n", nodes);
	puts("Adjacency list:");
	print_list(list, nodes);
	#endif

	int correct = ((argc - 2) <= nodes); // klik nie może być więcej niż wierzchołków

	#ifdef DEBUG
	printf("Cliques: %i\n", argc - 2);
	#endif
	for(int i = 2; correct && (i < argc); i++) {
		node_id_t length;
		node_id_t* clique = decode_clique(argv[i], &length);
		if(clique == NULL) {
			fputs("Błąd odczytu rozwiązania\n", stderr);
			free_list(list, nodes);
			return 1;
		}
		#ifdef DEBUG
		for(int i = 0; i < length; i++) {
			printf("%i ", clique[i]);
		}
		puts("");
		#endif
		correct = test_clique(clique, length, list, nodes);
		free(clique);
	}

	// rozwiązanie prawidłowe ⇔ graf pusty
	if(correct)
		correct = empty(list, nodes);

	#ifdef DEBUG
	printf("Nodes: %i\n", nodes);
	puts("Adjacency list:");
	print_list(list, nodes);
	#endif

	// for(int i = 0; i < argc; i++)
	//	  free(argv[i]);
	// free(argv);

	free_list(list, nodes);
	char* clique_count_b64 = encode_number(argc - 2);
	if(correct) 
		fputs("1 ", stdout);
	else
		fputs("0 ", stdout);
	puts(clique_count_b64);
	free(clique_count_b64);
}
