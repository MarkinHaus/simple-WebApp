use gloo::console::log;
use stylist::Style;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::Callback;
use yew::prelude::*;

const STYLE: &str = include_str!("text_input.css");

#[derive(Properties, PartialEq)]
pub struct PropsTextInput {
    pub class: String,
    pub id: String,
    pub name: String,
    pub type_: String,
    pub value: String,
    pub placeholder: String,
    pub label: String,
    pub set_value: Callback<String>,
    pub open: bool,
}


pub enum TextInputStagesMsg {
    SetValue(String),
    ToggleEdit,
}

pub struct TextInputField {
    pub value: String,
    pub edit: bool,
}

impl Component for TextInputField {
    type Message = TextInputStagesMsg;
    type Properties = PropsTextInput;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            value: "".to_string(),
            edit: ctx.props().open.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TextInputStagesMsg::SetValue(value) => {
                self.value = value.clone();
                ctx.props().set_value.emit(value);
                ctx.link().send_message(TextInputStagesMsg::ToggleEdit)
            }
            TextInputStagesMsg::ToggleEdit => {
                self.edit = !self.edit;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let onchange = Callback::from(move |event: Event| {
            let val = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            link.send_message(TextInputStagesMsg::SetValue(
                val.clone(),
            ));
            log!(val)
        });

        let link = ctx.link().clone();
        let toggle_edit = Callback::from(move |_| {
            link.send_message(TextInputStagesMsg::ToggleEdit);
        });

        let mut display = self.value.clone();
        if display.is_empty() {
            display = ctx.props().name.clone();
            display.push_str(" Input")
        }

        let stylesheet = Style::new(STYLE).unwrap();
        html! {
            <div class={stylesheet}>

                <div class="container">
                if self.edit.clone(){
                    <input
                        class={ctx.props().class.clone()}
                        id={ctx.props().id.clone()}
                        type={ctx.props().type_.clone()}
                        name={ctx.props().name.clone()}
                        placeholder={ctx.props().placeholder.clone()}
                        value={self.value.clone()}
                        onchange={onchange}
                    />
                } else {
                    <label onclick={toggle_edit}>{ctx.props().label.clone()}{" "}{&display}</label>
                }
                </div>

            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {}
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        // TODO ..
    }
}
