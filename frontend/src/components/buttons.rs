use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct LinkButtonProps {
    pub class: String,
    pub href: String,
    pub id: String,
    pub children: Html,
}

pub enum LinkButtonMsg {
    UpdateIsHovering,
}

pub struct LinkButton {
    is_hovering: bool,
}

impl Component for LinkButton {
    type Message = LinkButtonMsg;
    type Properties = LinkButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            is_hovering: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LinkButtonMsg::UpdateIsHovering => { self.is_hovering = !self.is_hovering; true },
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self { is_hovering, .. } = *self;
        let onhover = if is_hovering { "" } else { "" };
        
        html! {
            <>
                <a class={"relative flex items-center justify-center p-0.5 mt-[10px] rounded-lg group bg-gradient-to-br from-yellow-950 to-blue-900 group-hover:from-yellow-950 group-hover:to-blue-900"} href={ctx.props().href.clone()} id={ctx.props().id.clone()}>
                    <span class={classes!(ctx.props().class.clone(), "relative", "flex", "justify-center", "transition-all", "ease-in", "duration-75", "bg-gray-900", "rounded-md", "group-hover:bg-opacity-0",)}>
                        { ctx.props().children.clone() }
                    </span>
                </a>
            </>
        }
    }
}
