use yew::prelude::*;

pub struct Header;

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="row header">
                <div class="col-auto">
                    <span>{ "Aurora Alert" }</span>
                </div>
                <div class="col-auto">
                    <span>{ "2022" }</span>
                </div>
            </div>
        }
    }
}
