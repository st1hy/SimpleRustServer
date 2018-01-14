
function execute() {
    start_update_worker();
}

var worker;

function start_update_worker() {
    worker = new Worker('worker.js');
    worker.onmessage = function (msg) {
        var data = msg.data;
        if (data.operation == 'update') {
            document.getElementById(data.element).innerHTML = data.value;
        }
    }
    worker.onerror = function(e) {
        console.log('Error: Line ' + e.lineno + ' in ' + e.filename + ': ' + e.message);
    };
    worker.postMessage({operation: 'loop'});
}

function onResetPressed() {
    worker.postMessage({operation: 'reset'})
}
