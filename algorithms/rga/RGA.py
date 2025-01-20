import argparse
import base64
import random
# import requests

class CC_solver:
    def __init__(self):
        self.raw_input = ""
        self.encoded_graph = ""
        self.iterations = 1
        self.show_solution = False
        self.adj_matrix = None
        self.result_cliques = []

    def decode_graph(self):
        '''
        Decodes base64 encoded string representing graph into an adjacency matrix.
        
        Example:
            Input: "BgAONgo="
            Output: Adjacency matrix
        '''
        buff = base64.b64decode(self.encoded_graph)
        n = int.from_bytes(buff[:2], byteorder='little')
        self.adj_matrix = [[0]*n for _ in range(n)]

        index = 2
        mask = 1
        for i in range(n):
            for j in range(i, n):
                if (buff[index] & mask) != 0:
                    self.adj_matrix[i][j] = 1
                    self.adj_matrix[j][i] = 1

                mask <<= 1
                if mask == 256 and index < len(buff):
                    index += 1
                    mask = 1

    def parse_args(self):
        '''
        Parses arguments.
        Handles both rogal's string and path of file with rogal's string
        as an input
        '''
        # Parsing
        parser = argparse.ArgumentParser()

        parser.add_argument("-i", "--iterations", type=int, default=1, help="set number of iterations")
        parser.add_argument("-s", "--solution", action="store_true", help="show solution in readable form")

        group_input = parser.add_mutually_exclusive_group(required=True)
        group_input.add_argument("-r", "--raw", help="Provide a raw input string in Rogal's format")
        group_input.add_argument("-p", "--path", help="Provide path to file including Rogal's format string")
        group_input.add_argument("-n", "--net", nargs=2, type=int, metavar=('vertices_count', 'vertex_degree'),help="Provide ammount of graph vertices and vertex degree")

        args = parser.parse_args()
        self.iterations = args.iterations
        self.show_solution = args.solution

        # Handling input string
        if args.raw: # raw string is given as an input
            self.encoded_graph = args.raw
        if args.path: # path to file with string is given as an input
            with open(args.path, 'r') as input_file:
                self.encoded_graph = input_file.readline()
        if args.net: # request freshly generated graph from Rogal's server
            print("\'net\' functionality disabled to minimize dependencies")

            # nodes, degree = args.net[0], args.net[1]
            # if nodes % 2 != 0 and degree % 2 != 0:
            #     raise Exception("Wrong parameters values")
            
            # url = f"http://89.168.64.137:8080/gen_graph?nodes={nodes}&degree={degree}"
            # resp = requests.get(url)

            # json = resp.json()
            # self.encoded_graph = json["encodedGraph"]

    def find_solution(self):
        '''
        Finding solution to CC problem
        Random vertex is being chosen. If there is clique it can be added to, it will be.
        Otherwise new clique with this vertex will be created.
        '''
        def can_form_clique(clique, new_vertex):
            # Checks if a clique will still be a clique after adding new vertex to it
            return all(self.adj_matrix[i][new_vertex] == 1 for i in clique)

        buf_cliques = [] # temporary buffer for cliques
        vertices = list(range(len(self.adj_matrix)))

        while vertices: # repeating for all vertices in random order
            random_vertex = random.choice(vertices)
            found_matching_clique = False

            for i, clique in enumerate(buf_cliques):
                if can_form_clique(clique, random_vertex):
                    found_matching_clique = True
                    clique.append(random_vertex)
                    while i != 0:
                        if len(buf_cliques[i]) > len(buf_cliques[i-1]):
                            tmp = buf_cliques[i]
                            buf_cliques[i] = buf_cliques[i-1]
                            buf_cliques[i-1] = tmp
                            i -= 1
                        else:
                            break
                    break

            if not found_matching_clique:
                buf_cliques.append([random_vertex])

            vertices.remove(random_vertex)
        if not self.result_cliques:
            self.result_cliques = buf_cliques.copy()
        else:
            self.result_cliques = min(self.result_cliques, buf_cliques, key=len) # choosing answer containing least cliques

    def output_solution(self):
        '''
        Outputs solution in Jakub's format:
        [clique1] [clique2] ...
        '''
        def jakub_format(clique):
            n = len(clique)
            a = bytearray()
            a.extend(n.to_bytes(2, byteorder='little', signed=False))
            for vertex in sorted(clique):
                a.extend(vertex.to_bytes(2, byteorder='little', signed=False))
            return base64.b64encode(a).decode("utf-8")

        for clique in self.result_cliques:
            print(jakub_format(clique), end=' ')

    def run(self):
        '''
        Executes main logic of the program
        '''
        # handling input
        self.parse_args()
        self.decode_graph()

        # printing adjacency matrix
        if self.show_solution:
            print(f"Solution for graph:")
            for row in self.adj_matrix:
                print(*row)

        # solving given ammount of times
        for _ in range(self.iterations):
            self.find_solution()

        self.output_solution() # solution

        # more details about solution
        if self.show_solution:
            print(self.result_cliques)
            print(f"Len: {len(self.result_cliques)}")


if __name__ == '__main__':
    solver = CC_solver()
    solver.run()
