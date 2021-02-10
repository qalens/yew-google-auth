var googleUser = {};
var auth2;
export function render_with_callback(clientId,callback) {
    gapi.load('auth2', function(){
        // Retrieve the singleton for the GoogleAuth library and set up the client.
        auth2 = gapi.auth2.init({
            client_id: clientId,
            cookiepolicy: 'single_host_origin',
            // Request scopes in addition to 'profile' and 'email'
            //scope: 'additional_scope'
        });
        attachSignin(document.getElementById('login-with-google'),callback);
    });
};

function attachSignin(element,callback) {
    console.log(element.id);
    auth2.attachClickHandler(element, {},
        function(googleUser) {
            alert("Logged In for user "+googleUser.getBasicProfile().getName())
            console.log(googleUser.getAuthResponse(true));
            callback(googleUser.getAuthResponse(true))
        }, function(error) {
            alert(JSON.stringify(error, undefined, 2));
        });
}