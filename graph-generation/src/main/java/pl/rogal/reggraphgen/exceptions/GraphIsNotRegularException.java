package pl.rogal.reggraphgen.exceptions;

public class GraphIsNotRegularException extends RuntimeException {
    public GraphIsNotRegularException() {
        super("Graph is not regular.");
    }
}
