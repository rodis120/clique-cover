package pl.rogal.reggraphgen;

import pl.rogal.reggraphgen.exceptions.GraphTooLargeException;
import pl.rogal.reggraphgen.graphs.Graph;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.Base64;
import java.util.function.IntFunction;

public class BetterFormat {

    public static String serialize(Graph graph) {
        int n = graph.getNodeCount();
        if(n >= 0x10000) throw new GraphTooLargeException(n);

        int size = 2 + (n*(n+1)/2 + 7)/8;
        final ByteBuffer buffer = ByteBuffer.allocate(size);
        buffer.order(ByteOrder.LITTLE_ENDIAN);
        buffer.putShort((short)n);

        byte sum = 0;
        int bit = 1;
        for(int i = 0; i < n; i++) {
            for(int j = i; j < n; j++) {
                if(graph.hasEdge(i, j) || graph.hasEdge(j, i)) sum += (byte) bit;
                bit<<=1;
                if(bit == 256) {
                    buffer.put(sum);
                    bit = 1;
                    sum = 0;
                }
            }
        }
        if(sum != 0) buffer.put(sum);

        return Base64.getEncoder().encodeToString(buffer.array());
    }

    public static Graph deserialize(String str, IntFunction<Graph> constructor) {
        final ByteBuffer buff = ByteBuffer.wrap(Base64.getDecoder().decode(str));
        buff.order(ByteOrder.LITTLE_ENDIAN);
        int n = buff.getShort();

        Graph graph = constructor.apply(n);

        byte b = buff.get();
        int bit = 1;
        for(int i = 0; i < n; i++) {
            for(int j = i; j < n; j++) {
                if((b & bit) != 0) graph.addEdge(i, j);
                bit<<=1;
                if(bit == 256 && buff.hasRemaining()) {
                    b = buff.get();
                    bit = 1;
                }
            }
        }

        return graph;
    }
}
