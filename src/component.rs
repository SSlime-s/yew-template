use crate::css;
use yew::prelude::*;

pub struct Model {
    _link: ComponentLink<Self>,
}
pub enum Msg {}
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { _link: link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=css("font-size: 80px;")>
                {"Hello World !"}
            </div>
        }
    }
}
