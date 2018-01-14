function get_last_date(target) {
    httpGetAsync(window.location.origin + '/timestamp/PHONE', function (responseText) {
        console.log(responseText);
        var date = timeConverter(responseText)
        document.getElementById(target).innerHTML = responseText;
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
  return new Date(UNIX_timestamp * 1000).toISOString();
}
