package pl.rogal.reggraphgen.exceptions;

public class GraphSelfCycleException extends RuntimeException {
    public GraphSelfCycleException() {
        super("Self cycles are not supported.");
    }
}
