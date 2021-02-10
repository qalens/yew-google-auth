use yew::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use serde::{Serialize,Deserialize};
#[wasm_bindgen]
pub struct AuthResponse{
    access_token:String,
    id_token:String,
    login_hint:String,
    scope:String,
    expires_in:u64,
    first_issued_at:u64,
    expires_at:u64
}
#[wasm_bindgen]
impl AuthResponse {
    #[wasm_bindgen(getter)]
    pub fn access_token(&self) -> String {
        self.access_token.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_access_token(&mut self, access_token: String) {
        self.access_token = access_token;
    }

    #[wasm_bindgen(getter)]
    pub fn id_token(&self) -> String {
        self.id_token.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_id_token(&mut self, id_token: String) {
        self.id_token = id_token;
    }

    #[wasm_bindgen(getter)]
    pub fn login_hint(&self) -> String {
        self.login_hint.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_login_hint(&mut self, login_hint: String) {
        self.login_hint = login_hint;
    }

    #[wasm_bindgen(getter)]
    pub fn scope(&self) -> String {
        self.scope.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_scope(&mut self, scope: String) {
        self.scope = scope;
    }

    #[wasm_bindgen(getter)]
    pub fn expires_in(&self) -> u64 {
        self.expires_in.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_expires_in(&mut self, expires_in: u64) {
        self.expires_in = expires_in;
    }

    #[wasm_bindgen(getter)]
    pub fn first_issued_at(&self) -> u64 {
        self.first_issued_at.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_first_issued_at(&mut self, first_issued_at: u64) {
        self.first_issued_at = first_issued_at;
    }

    #[wasm_bindgen(getter)]
    pub fn expires_at(&self) -> u64 {
        self.expires_at.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_expires_at(&mut self, expires_at: u64) {
        self.expires_at = expires_at;
    }

}
pub struct Button{
    pub props:ButtonProps
}
#[derive(Clone,Properties)]
pub struct ButtonProps{
    pub client_id:String,
    pub on_login:Callback<AuthResponse>
}
impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self{ props}
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
                <div>
                    <div id="login-with-google">
                      <span>{"Sign In With Google"}</span>
                    </div>
                </div>
            }
    }

    fn rendered(&mut self, first_render: bool) {
        let callback = self.props.on_login.clone();
        crate::scripts::render_with_callback(self.props.client_id.clone(),Closure::once_into_js( move |resp:AuthResponse|{
            callback.emit(resp);
        }));
    }
}