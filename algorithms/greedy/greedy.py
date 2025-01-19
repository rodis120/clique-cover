import numpy as np
import random
import base64
import sys


def is_clique(vertices, adj_mat):
    for i in vertices:
        for j in vertices:
            if i != j and not adj_mat[i][j]:
                return False
    return True

def greedy_clique_cover(adj_mat):
    n = len(adj_mat)
    vertices = list(range(n))
    uncovered = set(vertices)
    cliques = []

    while uncovered:
        current_clique = []
        for v in list(uncovered):
            if is_clique(current_clique + [v], adj_mat):
                current_clique.append(v)
                uncovered.remove(v)
        cliques.append(current_clique)

    return cliques

def decode(s: str) -> list[list[int]]:
    byte_str = base64.b64decode(s)
    n = int.from_bytes(byte_str[:2], byteorder='little', signed=False)

    out = [[0]*n for _ in range(n)]

    index = 2
    bit = 1
    for i in range(n):
        for j in range(i, n):
            if((byte_str[index] & bit) != 0):
                out[i][j] = 1
                out[j][i] = 1

            bit <<= 1

            if(bit == 0x100):
                index += 1
                bit = 1

    return out

def encode_cliques(cliques):
    result = []

    for clique in cliques:
        byte_str = bytearray()
        byte_str.extend(len(clique).to_bytes(2, byteorder="little", signed=False))
        for vertex in clique:
            byte_str.extend(vertex.to_bytes(2, byteorder="little", signed=False))
        result.append(base64.b64encode(byte_str).decode("ascii"))

def encode(matrix: list[list[int]]) -> str:
    n = len(matrix)
    byte_str = bytearray()
    byte_str.extend(n.to_bytes(2, byteorder='little', signed=False))

    byte = 0
    bit = 1
    for i in range(n):
        for j in range(i, n):
            if matrix[i][j] == 1:
                byte += bit

            bit <<= 1

            if bit == 0x100:
                byte_str.append(byte)
                byte = 0
                bit = 1

    if len(byte_str) < 2 + ((n + 1) * n // 2 + 7) // 8:
        byte_str.append(byte)

    return base64.b64encode(byte_str).decode('ascii')

def encode_cliques(cliques):
    result = ""

    for clique in cliques:
        byte_str = bytearray()
        byte_str.extend(len(clique).to_bytes(2, byteorder="little", signed=False))
        for vertex in clique:
            byte_str.extend(vertex.to_bytes(2, byteorder="little", signed=False))
        result += (base64.b64encode(byte_str).decode("ascii")) + " "

    return result

def main():
    if len(sys.argv) < 2:
        sys.exit(1)

    graph_base64 = sys.argv[1]

    try:
        matrix = decode(graph_base64)
    except Exception as e:
        print(f"Invalid graph format: {e}")
        sys.exit(1)

    solution = greedy_clique_cover(matrix)
    encoded_cliques = encode_cliques(solution)

    print(encoded_cliques)

if __name__ == "__main__":
    main()
