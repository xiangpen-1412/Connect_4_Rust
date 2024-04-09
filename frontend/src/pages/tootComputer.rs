use yew::{Component, Context, Html};

pub struct TOOTComputer {
    game_started: bool,
    player_1_name: String,
    player_2_name: String,
    board_size: (i32, i32),
    difficulty_level: i32,
}

impl Component for TOOTComputer {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game_started: false,
            player_1_name: String::from(""),
            player_2_name: String::from("Computer"),
            board_size: (6,7),
            difficulty_level: 1,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        todo!()
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        todo!()
    }
}