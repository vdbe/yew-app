use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{components::{ChatInput, Comp}, services::TestService};

#[derive(Debug)]
pub enum Msg {
    SendChat(String),
}

pub struct Chat;

impl Chat {
    fn make_request(url: String) {
        spawn_local(async move {
            TestService::make_request(&url, true).await;
        });
    }
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SendChat(s) => {
                log::info!("{}", &s);
                Self::make_request(s);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_enter = ctx.link().callback(Msg::SendChat);

        html! {
             <ChatInput {on_enter} />
        }
    }
}
