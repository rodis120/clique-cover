function updateInput(inputElements, graphDist) {
    if (graphDist) {
        inputElements.numbers.nodesMin.value = graphDist.n_nodes_min;
        inputElements.numbers.nodesMax.value = graphDist.n_nodes_max;
        inputElements.numbers.nodesStep.value = graphDist.n_nodes_step;
        inputElements.numbers.nodeDensity.value = graphDist.node_density;
        inputElements.numbers.iterations.value = graphDist.n_iterations;
    }
}

let mutexGuard = false;

function setupInput(inputElements) {

    function ensureRange(element, fallback) {
        const step = parseFloat(element.step) || 1;
        const isFloat = step < 1;

        const min = isFloat ? 0.0 : 1;
        const max = isFloat ? 1.0 : 65535;

        element.addEventListener('input', () => {
            const value = isFloat ? parseFloat(element.value) : parseInt(element.value);
            if (isNaN(value)) {
                element.value = fallback;
            } else {
                element.value = Math.max(min, Math.min(max, value));
            }
        });
    }
    
    Object.keys(inputElements.sliders).forEach((key) => {
        console.log("inputElements's key:", key);
        const slider = inputElements.sliders[key];
        const number = inputElements.numbers[key];
        const fallback = inputElements.fallback[key];

        slider.addEventListener('input', () => {
            if (!mutexGuard) {
                mutexGuard = true;
                number.value = slider.value;
                mutexGuard = false;
            }
        });

        number.addEventListener('input', () => {
            if (!mutexGuard) {
                mutexGuard = true;
                slider.value = number.value;
                mutexGuard = false;
            }
        });

        ensureRange(number, fallback);
    });
}

function createRunRequest(inputElements) {

    const nodesMin = inputElements.numbers.nodesMin.value;
    const nodesMax = inputElements.numbers.nodesMax.value;
    const nodesStep = inputElements.numbers.nodesStep.value;
    const nodeDensity = inputElements.numbers.nodeDensity.value;
    const iterations = inputElements.numbers.iterations.value;

    const algosInUse = Array.from(inputElements.checkboxes)
        .filter(checkbox => checkbox.checked)
        .map(checkbox => parseInt(checkbox.id.replace('menu-algo-checkbox-', '')));

    const request = {
        RequestRestart: [
            "here would go the password if i had time to implement that functionality",
            {
                n_nodes_min: parseInt(nodesMin),
                n_nodes_max: parseInt(nodesMax),
                n_nodes_step: parseInt(nodesStep),
                node_density: parseFloat(nodeDensity),
                n_iterations: parseInt(iterations),
            },
            algosInUse,
        ],
    };
    
    return request;
}

function initChart(obj_id, title) {
    const chart = new Chart(
        document.getElementById(obj_id),
        {
            type: 'line',
            data: {
                labels: [],
                datasets: []
            },
            options: {
                responsive: true,
                plugins: {
                    title: {
                        display: true,
                        text: title
                    }
                }
            }
        }
    );

    return chart;
}

function getNodeCount(appState, graph_id) {
    return appState.graphs.map.get(graph_id).nodes
}

function updateChart(chart, appState, key) {
    const algorithms = new Map();
    for(const solution of appState.solutions.list) {
        if(!algorithms.has(solution.algo_id)) {
            algorithms.set(solution.algo_id, new Map());
        }

        const nodes = getNodeCount(appState, solution.graph_id);
        if(!algorithms.get(solution.algo_id).has(nodes)) {
            algorithms.get(solution.algo_id).set(nodes, []);
        }

        console.log(solution.result[key]);

        algorithms.get(solution.algo_id).get(nodes).push({
            nodes: nodes,
            value: solution.result[key],
        });
    }

    var labels = null;

    const datasets = []
    for(const [algo_id, algo] of algorithms) {
        const pairs = [];
        for(const [nodes, points] of algo) {
            const avg = points.reduce((acc, p) => acc + p, 0) / points.length;
            pairs.push({nodes: nodes, avg: avg});
        }
        pairs.sort((a, b) => a.nodes - b.nodes);
        
        if (labels == null) {
            labels = pairs.map(p => p.nodes);
        }

        datasets.push({
            label: appState.algorithms.names[algo_id],
            data: pairs.map(p => p.avg),
            fill: false,
        });
    }

    chart.data.labels = labels;
    chart.data.datasets = datasets;
    chart.update();
}

function createWebSocket(wsPath, inputElements, appState) {
    const wsProtocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsHost = window.location.host
    const wsUrl = `${wsProtocol}//${wsHost}/${wsPath}`;
    const socket = new WebSocket(wsUrl);
    console.log("websocket created");

    socket.addEventListener('open', () => {
        console.log('ws connection established');
    });

    socket.addEventListener('message', (event) => {
        let payload = event.data;
        console.log('received from ws:', payload);

        let msg = JSON.parse(event.data);
        let msgType = Object.keys(msg)[0];
        let msgPayload = msg[msgType];

        switch (msgType) {
            
            case 'AlgoList':
                appState.algorithms.names = msgPayload.reduce((acc, [id, name]) => {
                    acc[id] = name;
                    return acc;
                }, {});

                const algoListDiv = document.getElementById('menu-algo-list');
                algoListDiv.innerHtml = '';
                inputElements.checkboxes = [];
                Object.entries(appState.algorithms.names).forEach(([id, name]) => {
                    const div = document.createElement('div');
                    div.id = `menu-algo-${id}`;

                    const checkbox = document.createElement('input');
                    checkbox.type = 'checkbox';
                    checkbox.id = `menu-algo-checkbox-${id}`;
                    checkbox.checked = true;
                    inputElements.checkboxes.push(checkbox);

                    const label = document.createElement('h5');
                    label.textContent = name;

                    div.appendChild(checkbox);
                    div.appendChild(label);

                    algoListDiv.appendChild(div);
                });

                break;

            case 'AlgosInUse':
                appState.algorithms.inUse = msgPayload;
                
                const areUsed = Object.keys(appState.algorithms.names)
                    .map(key => false);

                appState.algorithms.inUse.forEach((id) => {
                    areUsed[id] = true;
                });
                

                areUsed.forEach((val, id) => {
                    const checkbox = document.getElementById(`menu-algo-checkbox-${id}`);
                    checkbox.checked = val;
                });

                break;

            case 'Graph':
                var graph_id = msgPayload[1];
                var encoded_graph = msgPayload[2]["inner"];

                var buffer = Uint8Array.fromBase64(encoded_graph);
                var nodes = buffer[0] + buffer[1] * 0x100;

                appState.graphs.map.set(graph_id, {
                    graph_id: graph_id,
                    nodes: nodes,
                    encoded_graph: encoded_graph
                });

                break;

            case 'Solution':
                var algo_id = msgPayload[1];
                var graph_id = msgPayload[2];
                var result = msgPayload[3]

                appState.solutions.list.push({
                    algo_id: algo_id,
                    graph_id: graph_id,
                    result: result,
                })

                updateChart(appState.perf_chart, appState, 'n_cpu_cycles');
                updateChart(appState.quality_chart, appState, 'n_cliques');

                break;
        }
    });

    socket.addEventListener('error', (error) => {
        console.error('ws error', error);
    });
    
    return socket;
}

document.addEventListener('DOMContentLoaded', () => {
    const inputElements = {
        checkboxes: {
        },
        sliders: {
            nodesMin: document.getElementById('slider-nodes-min'),
            nodesMax: document.getElementById('slider-nodes-max'),
            nodesStep: document.getElementById('slider-nodes-step'),
            nodeDensity: document.getElementById('slider-node-density'),
            iterations: document.getElementById('slider-iterations'),
        },
        numbers: {
            nodesMin: document.getElementById('number-nodes-min'),
            nodesMax: document.getElementById('number-nodes-max'),
            nodesStep: document.getElementById('number-nodes-step'),
            nodeDensity: document.getElementById('number-node-density'),
            iterations: document.getElementById('number-iterations'),
        },
        fallback: {
            nodesMin: 100,
            nodesMax: 1000,
            nodesStep: 1,
            nodeDensity: 0.2,
            iterations: 5,
        },
    };

    const appState = {
        running: false,
        algorithms: {
        },
        graphs: {
            map: new Map(),
        },
        solutions: {
            list: [],
        },
        perf_chart: initChart('performance_chart', 'Time'),
        quality_chart: initChart('quality_chart', 'Qliques'),
    };

    const socket = createWebSocket('ws', inputElements, appState);

    setupInput(inputElements);

    let launchButton = document.getElementById('launch-button');

    launchButton.addEventListener('click', () => {
        if (socket.readyState === WebSocket.OPEN) {
            console.log("Launch Button clicked");
            if (!appState.running) {
                const message = createRunRequest(inputElements);
                socket.send(JSON.stringify(message));
                console.log("Sending the following message to the server:", message);
            }
        }
    });

});
