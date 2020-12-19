pub use pulldown_cmark;
use pulldown_cmark::{Options, Parser};
use wasm_bindgen::JsCast;
use yew::{Component, ComponentLink, Html, Properties};
use std::rc::Rc;
fn default_options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options
}

#[derive(Properties, Clone)]
pub struct Props {
    pub content: String,
    #[prop_or_else(default_options)]
    pub options: Options,
}

pub struct Markdown {
    props: Props,
}

impl Component for Markdown {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, mut props: Self::Properties) -> bool {
        std::mem::swap(&mut props, &mut self.props);
        true
    }

    fn view(&self) -> Html {
        let mut parser = Parser::new_ext(&self.props.content, self.props.options);

        let mut html_output = String::with_capacity(self.props.content.len() * 3 / 2);
        pulldown_cmark::html::push_html(&mut html_output, parser);

        html_as_vdom(&html_output)
    }
}

fn html_as_vdom(content: &str) -> Html {
    let element = yew::utils::document().create_element("div").unwrap();
    element.set_inner_html(content);
    let children = element.children();
    weblog::console_log!(&element);

    let children = js_sys::Array::from(&children);
    let children = children.to_vec().into_iter().map(|child| {
        let child = child.dyn_into::<web_sys::Element>();
        Html::VRef(child.unwrap().into())
    });

    yew::html! { for children }
}
