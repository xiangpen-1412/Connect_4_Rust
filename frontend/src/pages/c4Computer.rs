use web_sys::wasm_bindgen::JsCast;
use yew::{Component, Context, function_component, Html, html, InputEvent};

pub struct Connect4Computer {
    game_started: bool,
    player_name: String,
    difficulty_level: String
}

pub enum Msg {
    GameStarted(String),
}

#[function_component(GameBoard)]
fn game_board() -> Html {

    html! {
        <div>
        
        </div>
    }
}

impl Component for Connect4Computer {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game_started: false,
            player_name: String::from(""),
            difficulty_level: "Easy".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GameStarted(player) => {
                self.game_started = true;
                self.player_name = player;
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        unimplemented!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| {
            let input = gloo_utils::document().get_element_by_id("textbox1").unwrap();
            let input: web_sys::HtmlInputElement = input.dyn_into::<web_sys::HtmlInputElement>().unwrap();
            let name = input.value();
            Msg::GameStarted(name)
        });

        html! {
        <div class="main" style="margin-left: 390px; margin-right: 40px">
            <div class="w3-container" id="services" style="margin-top: 75px">
                <h5 class="w3-xxxlarge" style="color: violet"><b>{"Enter Player Names"}</b></h5>
                <hr style="width: 50px; border: 5px solid violet" class="w3-round" />
            </div>

            <div class="col-md-offset-4 col-md-8" style="margin-left: 10px; margin-top: 20px">
                <input id="textbox1" type="text" placeholder="Your Name" />
                <button id="startbutton" class="button" onclick={onclick}>{ "Start Game" }</button>
            </div>

            <div style="margin-left: 10px; margin-top: 20px">
                <label for="choose_level"> {"Choose difficulty level:"} </label>
                <select id="choose_level" style="margin: 3px">
                    <option selected=true disabled=false> {"Easy"} </option>
                    <option selected=false disabled=false> {"Hard"} </option>
                </select>
            </div>

            if self.game_started {
                <div style="margin-left: 10px; margin-top: 40px">
                    <h4>{format!("New Game: {} Vs AlphaCat", self.player_name)}</h4>
                    <small>{format!("(Disc Colors: {} - ", self.player_name)}<b style="color: red">{"Red"}</b>{" and AlphaPuff - "}<b style="color: yellow">{"Yellow"}</b>{")"}</small>

                    <GameBoard />
                </div>
            }

        </div>
        }
    }
}