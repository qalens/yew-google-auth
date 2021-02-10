use yew::prelude::*;
use wasm_bindgen::closure::Closure;

use serde::{Serialize,Deserialize};
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct AuthResponse{
    pub access_token:String,
    pub id_token:String,
    pub login_hint:String,
    pub scope:String,
    pub expires_in:u64,
    pub first_issued_at:u64,
    pub expires_at:u64
}

pub struct Button{
    pub props:ButtonProps
}
#[derive(Clone,Properties)]
pub struct ButtonProps{
    pub client_id:String,
    pub on_login:Callback<String>
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
        crate::scripts::render_with_callback(self.props.client_id.clone(),Closure::once_into_js( move |token:String|{
            callback.emit(token);
        }));
    }
}