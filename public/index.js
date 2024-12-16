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

function createWebSocket(wsPath, inputElements) {
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

    });

    socket.addEventListener('error', (error) => {
        console.error('ws error', error);
    });
    
    return socket;
}

document.addEventListener('DOMContentLoaded', () => {
    const inputElements = {
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

    const socket = createWebSocket('ws', inputElements);

    setupInput(inputElements);

    let launchButton = document.getElementById('button-launch');

    launchButton.addEventListener('click', () => {
        console.log('socket state:', socket.readyState);
        if (socket.readyState === WebSocket.OPEN) {
        }
    });

});
