package pl.rogal.reggraphgen.exceptions;

public class GraphTooLargeException extends RuntimeException {
    public GraphTooLargeException(int nodes) {
        super("Graph too large (nodes: %d)".formatted(nodes));
    }
}
