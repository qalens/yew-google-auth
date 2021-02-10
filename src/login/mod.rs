use yew::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use serde::{Serialize,Deserialize};
#[derive(Clone,Serialize,Deserialize)]
pub struct AuthResponse{
    access_token:String,
    id_token:String,
    login_hint:String,
    scope:String,
    expires_in:u64,
    first_issued_at:u64,
    expires_at:u64
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
        crate::scripts::render_with_callback(self.props.client_id.clone(),Closure::once_into_js( move |resp:JsValue|{
            callback.emit(resp.into_serde().unwrap());
        }));
    }
}