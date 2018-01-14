var loop = false;
var update_rate = 60000;

self.onmessage = function(msg) {
    switch (msg.data.operation) {
        case "loop":
            update_time();
            loop = true;
            setTimeout(update_time, update_rate);
            break;
        case "reset":
            onReset();
            break;
    }
}

function update_time() {
    get_last_date('last_drop_time');
    if (loop) setTimeout(update_time, update_rate);
}

function get_last_date(target) {
    httpGetAsync(location.origin + '/timestamp/PHONE', function (responseText) {
        var date = timeConverter(responseText)
        self.postMessage({
                operation: 'update',
                element: target,
                value: date,
            });
    })
}

function httpGetAsync(theUrl, callback) {
    var xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        var DONE = 4; // readyState 4 means the request is done.
        var OK = 200; // status 200 is a successful return.
        if (xhr.readyState == DONE) {
            if (xhr.status == OK) {
                callback(xhr.responseText);
            } else {
                console.log('Http async GET error: ' + xhr.status);
            }
        }
    }
    xhr.open("GET", theUrl, true); // true for asynchronous
    xhr.send(null);
}

function timeConverter(UNIX_timestamp) {
  var date = new Date(UNIX_timestamp * 1000);
  return date.toLocaleDateString() + " " + date.toLocaleTimeString();
}

function onReset() {
    postAsync("/echo", {operation: "reset"}, function(msg) {

        console.log("onReset returned");
    });
}

function postAsync(url, data, callback) {
    var xhr = new XMLHttpRequest();
    xhr.open("POST", url, true);
    console.log("onReset");

    //Send the proper header information along with the request
    xhr.setRequestHeader("Content-type", "application/x-www-form-urlencoded");

    xhr.onreadystatechange = function() {
        var DONE = 4; // readyState 4 means the request is done.
        var OK = 200; // status 200 is a successful return.
        if (xhr.readyState == DONE) {
            if (xhr.status == OK) {
                console.log(xhr.responseText);
                callback(xhr.responseText);
            } else {
                console.log('Http async POST error: ' + xhr.status);
            }
        }
    }
    xhr.send(data);
}