mod pages;

use yew::html::Scope;
use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch};
use yew_router::prelude::Link;
use crate::pages::c4Computer::Connect4Computer;
use crate::pages::homePage::Home;
use crate::pages::howC4::HowToConnect4;
use crate::pages::howToot::HowToTOOT;
use crate::pages::tootComputer::TOOTComputer;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/c4Computer")]
    Connect4Computer,
    #[at("/howC4")]
    HowToConnect4,
    #[at("/howToot")]
    HowToToot,
    #[at("/tootComputer")]
    TOOTComputer,
}

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        unimplemented!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
                <h3 class="w3-padding-64"><b>{"Play"}</b><b>{"Connect4 / TOOT-OTTO"}</b></h3>
            </BrowserRouter>
        }
    }
}

impl App {
    fn view_nav(&self, _link: &Scope<Self>) -> Html {
        html! {
            <nav class="w3-sidenav w3-collapse w3-top w3-large w3-padding"
                style="z-index:3;width:350px;font-weight:bold;background:violet" id="mySidenav"><br/>
                <a href="javascript:void(0)" class="w3-padding-xlarge w3-hide-large w3-display-topleft w3-hover-white"
                style="width:100%">{"Close Menu"}</a>
                <div class="w3-container">
                    <h3 class="w3-padding-64"><b>{"Play"}<br/>{"Connect4 / TOOT-OTTO"}</b></h3>
                </div>
                <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                <br/>
                <Link<Route> to={Route::HowToConnect4}>{ "How To Play Connect4" }</Link<Route>>
                <Link<Route> to={Route::Connect4Computer}>{ "Play Connect4 With Computer" }</Link<Route>>
                <br/>

                <Link<Route> to={Route::HowToToot}>{ "How to Play TOOT-OTTO" }</Link<Route>>
                <Link<Route> to={Route::TOOTComputer}>{ "Play Toot-Otto With Computer" }</Link<Route>>
            </nav>
        }
    }
}
fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Home => html! { <Home/> },
        Route::Connect4Computer => html! { <Connect4Computer/> },
        Route::HowToConnect4 => html! { <HowToConnect4/> },
        Route::HowToToot => html!{ <HowToTOOT/> },
        Route::TOOTComputer => html! { <TOOTComputer/> },
    }
}

fn main() {
    yew::start_app::<App>();
}
