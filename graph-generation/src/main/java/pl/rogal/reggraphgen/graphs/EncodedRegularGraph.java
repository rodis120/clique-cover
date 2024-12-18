package pl.rogal.reggraphgen.graphs;

import pl.rogal.reggraphgen.exceptions.GraphIsNotRegularException;

public class EncodedRegularGraph extends EncodedGraph {

    private final int degree;

    public EncodedRegularGraph(Graph graph, int degree) {
        super(graph);
        this.degree = degree;

        if(!RegularGraphGenerators.isRegular(graph, degree)) throw new GraphIsNotRegularException();
    }

    public int getDegree() {
        return degree;
    }
}
