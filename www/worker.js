var loop = false;

self.onmessage = function(msg) {
    console.log("Worker: " + msg.data);
    if (msg.data.operation == "loop") {
        update_time();
        loop = true;
        setTimeout(update_time, 1000);
    }
}

function update_time() {
    console.log('update time running');
    get_last_date('last_drop_time');
    if (loop) setTimeout(update_time, 1000);
}

function get_last_date(target) {
    httpGetAsync(location.origin + '/timestamp/PHONE', function (responseText) {
        console.log(responseText);
        var date = timeConverter(responseText)
        self.postMessage({
                operation: 'update',
                element: target,
                value: date,
            });
    })
}

function httpGetAsync(theUrl, callback)
{
    var xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        var DONE = 4; // readyState 4 means the request is done.
        var OK = 200; // status 200 is a successful return.
        if (xhr.readyState == DONE) {
            if (xhr.status == 200) {
                callback(xhr.responseText);
            } else {
                console.log('Http async get error: ' + xhr.status);
            }
        }
    }
    console.log(xhr);
    xhr.open("GET", theUrl, true); // true for asynchronous
    xhr.send(null);
}

function timeConverter(UNIX_timestamp) {
  var date = new Date(UNIX_timestamp * 1000);
  return date.toLocaleDateString() + " " + date.toLocaleTimeString();
}
