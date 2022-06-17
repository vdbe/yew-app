use web_sys::HtmlInputElement;
use yew::{events::KeyboardEvent, html::IntoPropValue, prelude::*};

pub enum Msg {
    Submit(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_enter: Callback<String>,
}

#[derive(Debug, Default)]
pub struct ChatInput;

impl Component for ChatInput {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit(value) => {
                ctx.props().on_enter.emit(value);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeypress =
            ctx.link().batch_callback(move |event: KeyboardEvent| {
                if event.key() == "Enter" {
                    let input: HtmlInputElement = event.target_unchecked_into();
                    let value = input.value();
                    input.set_value("");
                    Some(Msg::Submit(value))
                } else {
                    None
                }
            });

        html! {
            <input type="text" {onkeypress} />
        }
    }
}
