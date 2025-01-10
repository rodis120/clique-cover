package pl.rogal.reggraphgen.exceptions;

public class ImpossibleRegularGraphException extends RuntimeException {
    public ImpossibleRegularGraphException() {
        super("Regular Graph with given parameter is impossible.");
    }
}

