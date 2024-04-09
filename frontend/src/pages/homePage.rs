use yew::{Component, Context, Html, html};

pub struct Home;

impl Component for Home {
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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="main" style="margin-left:390px;margin-right:40px">
                <form>
                    <div class="w3-container" id="services" style="margin-top:75px">
                        <h5 class="w3-xxxlarge" style="color:violet"><b>{"Welcome"}</b></h5>
                        <hr style="width:50px;border:5px solid violet" class="w3-round"/>
                        <p>{"This application contains the following two board games in human Vs. Computer version."}
                        </p>
                        <ul>
                            <li>{"Connect 4"}</li>
                            <li>{"TOOT-OTTO"}</li>
                        </ul>
                        <p>{"Select the game of your choice from the side bar, and start playing. Enjoy!"}</p>
                    </div>
                </form>
            </div>
        }
    }
}