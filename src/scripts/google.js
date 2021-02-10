var googleUser = {};
var auth2;
export function render_with_callback(clientId,callback) {
    gapi.load('auth2', function(){
        // Retrieve the singleton for the GoogleAuth library and set up the client.
        auth2 = gapi.auth2.init({
            client_id: clientId,
            cookiepolicy: 'single_host_origin'
        });
        attachSignin(document.getElementById('login-with-google'),callback);
    });
};

function attachSignin(element,callback) {
    auth2.attachClickHandler(element, {},
        function(googleUser) {
            callback(googleUser.getAuthResponse(true))
        }, function(error) {
            alert(JSON.stringify(error, undefined, 2));
        });
}