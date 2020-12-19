use yew::prelude::*;
use yew_md::Markdown;
use yew_md::pulldown_cmark::Options;

struct App {

}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
let input = "
This is an abbr.
";

html! {
    <Markdown content=input options=Options::all() />
}
    }
}

fn main() {
    yew::start_app::<App>()
}
