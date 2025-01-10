package pl.rogal.reggraphgen;

import com.google.gson.Gson;
import com.google.gson.JsonObject;
import pl.rogal.reggraphgen.exceptions.ImpossibleRegularGraphException;
import pl.rogal.reggraphgen.graphs.EncodedRegularGraph;
import pl.rogal.reggraphgen.graphs.Graph;
import pl.rogal.reggraphgen.graphs.RegularGraphGenerators;
import pl.rogal.reggraphgen.graphs.AdjacencyMatrixGraph;

import static spark.Spark.*;

public class RestApi {

    private static final Gson GSON = new Gson();

    public static void startRestApi(int port) {
        port(port);

        get("/gen_graph", "application/json", (req, resp) -> {
            int n = Integer.parseInt(req.queryParams("nodes"));
            int r = Integer.parseInt(req.queryParams("degree"));

            Graph graph = RegularGraphGenerators.genregTL(n, r, AdjacencyMatrixGraph::new);
            EncodedRegularGraph encodedGraph = new EncodedRegularGraph(graph, r);

            resp.header("Content-Type", "application/json");
            return GSON.toJson(encodedGraph);
        });

        exception(ImpossibleRegularGraphException.class, (e, req, resp) -> {
            resp.header("Content-Type", "application/json");
            JsonObject root = new JsonObject();
            root.addProperty("error", "Cannot create graph with given parameters.");
            resp.body(GSON.toJson(root));
        });
    }
}
