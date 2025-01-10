package pl.rogal.reggraphgen.graphs;

import java.util.Collection;
import java.util.Iterator;

public interface Graph {

    void addEdge(int from, int to);
    void removeEdge(int from, int to);
    boolean hasEdge(int from, int to);

    Collection<Integer> getEdges(int nodeId);
    int getNodeDegree(int n);

    Iterator<Integer> getEdgesIterator(int nodeId);

    int getNodeCount();
    int getEdgeCount();
}
