package pl.rogal.reggraphgen.graphs;

import pl.rogal.reggraphgen.exceptions.ImpossibleRegularGraphException;
import pl.rogal.reggraphgen.utils.Pair;

import java.util.*;
import java.util.function.IntFunction;
import java.util.stream.Stream;

public class RegularGraphGenerators {

    private static final Random RANDOM = new Random();

    public static Graph genregTL(int n, int r, IntFunction<Graph> constructor) {
        if((n % 2 != 0 && r % 2 != 0) || n <= r) throw new ImpossibleRegularGraphException();

        final int[] deg = new int[n];
        final Edge[] edges = new Edge[n*r/2];
        int edgesIndex = 0;
        final Graph graph = constructor.apply(n);
        boolean shuffle = true;

        for(int i = 0; i < n; i++) {
            {
                List<Integer> nodes = new ArrayList<>(n-i);
                for(int j = i+1; j < n; j++)
                    if(deg[j] < r && !graph.hasEdge(i, j)) nodes.add(j);
                Collections.shuffle(nodes, RANDOM);

                Iterator<Integer> it = nodes.iterator();
                while(deg[i] < r && it.hasNext()) {
                    int j = it.next();

                    deg[i]++;
                    deg[j]++;
                    edges[edgesIndex++] = new Edge(i, j);
                    graph.addEdge(i, j);
                }
                if(deg[i] == r) continue;
            }

            //simple trick gain huge performance improvement
            if(shuffle) {
                shuffle(edges, edgesIndex);
                shuffle = false;
            }
            int index = 0;
            while(deg[i] < r-1) {
                Edge edge;
                int x, y;
                do {
                    edge = edges[index++];
                    x = edge.a;
                    y = edge.b;
                } while(x == i || y == i || graph.hasEdge(x, i) || graph.hasEdge(y, i));

                edges[index-1] = new Edge(x, i);
                edges[edgesIndex++] = new Edge(y, i);

                graph.removeEdge(x, y);
                graph.addEdge(x, i);
                graph.addEdge(y, i);

                deg[i] += 2;
            }

            if(deg[i] == r-1) {
                List<Integer> nodes = new ArrayList<>(n-i);
                for(int j = i+1; j < n; j++)
                    if(deg[j] < r) nodes.add(j);
                Collections.shuffle(nodes, RANDOM);

                final int x = i;
                List<Edge> subEdges = Arrays.stream(edges, 0, edgesIndex)
                        .filter(e -> e.a != x && e.b != x && !(graph.hasEdge(e.a, x) && graph.hasEdge(e.b, x)))
                        .toList();

                Stream<Pair<Integer, Edge>> pairs = nodes.stream()
                        .flatMap(node -> subEdges.stream()
                                .filter(e -> e.a != node && e.b != node && !(graph.hasEdge(e.a, node) && graph.hasEdge(e.b, node)))
                                .map(e -> new Pair<>(node, e))
                        );

                int u = -1, w = -1, v = -1;
                Edge edge = null;
                for(Pair<Integer, Edge> pair : (Iterable<Pair<Integer, Edge>>) pairs::iterator) {
                    v = pair.left();
                    edge = pair.right();
                    if(!graph.hasEdge(i, edge.a) && !graph.hasEdge(v, edge.b)) {
                        u = edge.a;
                        w = edge.b;
                        break;
                    } else if(!graph.hasEdge(i, edge.b) && !graph.hasEdge(v, edge.a)) {
                        u = edge.b;
                        w = edge.a;
                        break;
                    }
                }
                if(u == -1 || w == -1) {
                    continue;
                }

                int j = find(edges, edge);
                edges[j] = new Edge(i, u);
                edges[edgesIndex++] = new Edge(v, w);

                graph.removeEdge(edge.a, edge.b);
                graph.addEdge(i, u);
                graph.addEdge(v, w);

                deg[i]++;
                deg[v]++;
            }
        }

        return graph;
    }

    public static boolean isRegular(Graph graph, int r) {
        for(int i = 0; i < graph.getNodeCount(); i++) {
            if(graph.getNodeDegree(i) != r) return false;
        }

        return true;
    }

    private static void shuffle(Object[] array, int len) {
        for(int i = 0; i < len; i++) {
            int index = RANDOM.nextInt(len);
            swap(array, i, index);
        }
    }

    private static <T> int find(T[] array, T obj) {
        for(int i = 0; i < array.length; i++) {
            if(array[i].equals(obj)) return i;
        }

        return -1;
    }

    private static void swap(Object[] arr, int a, int b) {
        Object o = arr[a];
        arr[a] = arr[b];
        arr[b] = o;
    }

    private record Edge(int a, int b) {
        private Edge(int a, int b) {
            if (a < b) {
                this.a = a;
                this.b = b;
            } else {
                this.a = b;
                this.b = a;
            }
        }
    }
}
