package pl.rogal.reggraphgen.graphs;

import pl.rogal.reggraphgen.BetterFormat;

public class EncodedGraph {

    private final int nodes;
    private final String encodedGraph;
    private final int size;

    public EncodedGraph(Graph graph) {
        this.nodes = graph.getNodeCount();
        this.encodedGraph = BetterFormat.serialize(graph);
        this.size = encodedGraph.length();
    }

    public int getNodes() {
        return nodes;
    }

    public String getEncodedGraph() {
        return encodedGraph;
    }

    public int getSize() {
        return size;
    }
}
