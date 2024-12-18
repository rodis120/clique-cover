package pl.rogal.reggraphgen.graphs;

import pl.rogal.reggraphgen.exceptions.GraphSelfCycleException;

import java.util.*;

public class AdjacencyMatrixGraph implements Graph {

    private final int[][] matrix;
    private final int[] degrees;
    private final int nodeCount;
    private int edgeCount = 0;

    public AdjacencyMatrixGraph(int nodes) {
        nodeCount = nodes;
        matrix = new int[nodes][nodes];
        degrees = new int[nodes];
        for(int i = 0; i < nodes; i++) for(int j = 0; j < nodes; j++) matrix[i][j] = 0;
    }

    @Override
    public void addEdge(int from, int to) {
        if(from == to) throw new GraphSelfCycleException();
        if(hasEdge(from, to)) return;

        matrix[from][to] = 1;
        matrix[to][from] = 1;
        degrees[from]++;
        degrees[to]++;
        edgeCount++;
    }

    @Override
    public void removeEdge(int from, int to) {
        if(!hasEdge(from, to)) return;

        matrix[from][to] = 0;
        matrix[to][from] = 0;
        degrees[from]--;
        degrees[to]--;
        edgeCount--;
    }

    @Override
    public boolean hasEdge(int from, int to) {
        return matrix[from][to] == 1;
    }

    @Override
    public Collection<Integer> getEdges(int nodeId) {
        List<Integer> edges = new ArrayList<>(nodeCount);

        for(int i = 0; i < nodeCount; i++) {
            if(matrix[nodeId][i] != 0) edges.add(i);
        }

        return edges;
    }

    @Override
    public int getNodeDegree(int n) {
        return degrees[n];
    }

    @Override
    public Iterator<Integer> getEdgesIterator(int nodeId) {
        return new EdgeIterator(nodeId);
    }

    @Override
    public int getNodeCount() {
        return nodeCount;
    }

    @Override
    public int getEdgeCount() {
        return edgeCount;
    }

    private class EdgeIterator implements Iterator<Integer> {

        private final int node;
        private int next = -1;
        private boolean hasNext = false;

        public EdgeIterator(int node) {
            this.node = node;

            findNext();
        }

        @Override
        public boolean hasNext() {
            return hasNext;
        }

        @Override
        public Integer next() {
            if(!hasNext) throw new NoSuchElementException();

            int out = next;
            findNext();

            return out;
        }

        private void findNext() {
            hasNext = false;
            for(int i = next+1; i < nodeCount; i++) {
                if(matrix[node][i] == 1) {
                    next = i;
                    hasNext = true;
                    break;
                }
            }
        }
    }
}
