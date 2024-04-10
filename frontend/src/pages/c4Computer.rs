use gloo_utils::format::JsValueSerdeExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, window, Request, Headers, RequestInit, RequestMode, Response};
use yew::{Callback, Component, Context, function_component, Html, html, Properties, use_effect_with_deps, use_state, use_state_eq, UseStateHandle};

pub struct Connect4Computer {
    game_started: bool,
    player_name: String,
    difficulty_level: String
}

pub enum Msg {
    GameStarted(String),
}

#[derive(Properties, PartialEq)]
pub struct GameBoardProps {
    pub difficulty_level: String,
    pub player_name: String,
}

// pub fn alphaPuff_turn(game_board_state: &UseStateHandle<Vec<Vec<i64>>>, player_turn: &UseStateHandle<bool>) {
//     let game_board_state = game_board_state.clone();
//     let player_turn = player_turn.clone();
//     let player_clone = (*player_turn).clone();
//
//     if !player_clone {
//         for col in 0..7 {
//             let mut game_board_clone = (*game_board_state).clone();
//             web_sys::console::log_1(&format!("Game: {:?}", game_board_clone).into());
//             for row in 0..6 {  // Check from the bottom of the column
//                 if game_board_clone[row][col] == 0 {
//                     game_board_clone[row][col] = 2; // Assuming 2 for computer's disc
//                     game_board_state.set(game_board_clone);
//                     player_turn.set(true); // Switch back to player's turn after computer's move
//                     break;
//                 }
//             }
//         }
//     }
// }

fn check_winner(game_board: Vec<Vec<i64>>) -> i64 {

    let rows = game_board.len();
    let cols = game_board[0].len();
    // four directions
    let directions = [(0, 1), (1, 0), (1, 1), (-1, 1)];

    for x in 0..rows {
        for y in 0..cols {
            if game_board[x][y] != 0 {
                for dir in directions.iter() {
                    let mut count = 0;
                    let (dx, dy) = *dir;
                    while count < 4 {
                        let new_x = (x as i64) + (dx * count);
                        let new_y = (y as i64) + (dy * count);
                        if new_x < 0 || new_x >= rows as i64 || new_y < 0 || new_y >= cols as i64 {
                            break;
                        }
                        if game_board[new_x as usize][new_y as usize] != game_board[x][y] {
                            break;
                        }
                        count += 1;
                    }
                    if count == 4 {
                        return game_board[x][y];
                    }
                }
            }
        }
    }

    // check draw
    let is_draw = game_board.iter().all(|row| row.iter().all(|&cell| cell != 0));
    if is_draw {
        return 3; // return if draw
    }

    0 // no winner yet
}

async fn post_game_move(game_board: Vec<Vec<i64>>, difficulty_level: String) -> Result<i64, reqwest::Error> {
    web_sys::console::log_1(&format!("Enter post_game_move").into());
    let client = Client::new();
    let res = client.post("http://127.0.0.1:4869/api/puff_step")
        .json(&json!({"game_board": game_board, "difficulty_level": difficulty_level}))
        .send()
        .await?;

    let colResponse: serde_json::Value = res.json().await?;
    Ok(colResponse["column"].as_i64().unwrap_or_default())
}

pub fn alphaPuff_turn(game_board_state: &UseStateHandle<Vec<Vec<i64>>>, player_turn: &UseStateHandle<bool>, difficulty_level: String) {
    let game_board_state = game_board_state.clone();
    let player_turn = player_turn.clone();
    let player_clone = (*player_turn).clone();

    if !player_clone {
        let game_board_clone = (*game_board_state).clone();
        let mut new_board = game_board_clone.clone();
        spawn_local(async move {
            web_sys::console::log_1(&format!("Enter").into());
            match post_game_move(game_board_clone, difficulty_level.clone()).await {
                Ok(column) => {
                    let mut row = 6;
                    while row >= 0 {
                        row -= 1;
                        if new_board[row][column as usize] == 0 {
                            new_board[row][column as usize] = 2;
                            break;
                        }
                    }
                    game_board_state.set(new_board);
                    player_turn.set(true);
                    web_sys::console::log_1(&format!("Received column: {}", column).into());
                }
                Err(error) => {
                    web_sys::console::log_1(&format!("HTTP Request failed: {:?}", error).into());
                }
            }
        })
    }
}

#[function_component(GameBoard)]
fn game_board(props: &GameBoardProps) -> Html {
    let game_board_state = use_state(|| vec![vec![0; 7]; 6]);
    let player_turn = use_state(|| true);
    let game_is_over = use_state(|| false);
    let player_name = use_state(|| props.player_name.clone());
    let winner = use_state(|| "A".to_string());
    let difficulty_level = use_state(|| props.difficulty_level.clone());
    let game_board_state_clone = game_board_state.clone();
    let player_turn_clone = player_turn.clone();
    let game_is_clone_over = game_is_over.clone();
    let clone_winner = winner.clone();
    let game_is_over_clone_clone = game_is_over.clone();

    let onClickHandle = {
        let game_board_state = game_board_state.clone();
        let player_turn = player_turn.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            // if not player turn, ignore
            if (!*player_turn) {
               return;
            }

            let document = window().unwrap().document().unwrap();
            let canvas = document.get_element_by_id("gameBoard").unwrap();
            let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();

            // get canvas context
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            let rect = canvas.get_bounding_client_rect();
            let x = event.client_x() as f64 - rect.left();

            // Determine which column was clicked
            let column = ((x - 100.0) / 75.0).round() as usize;

            // update the state map based on the clicked column
            if column >= 0 && column < 7 {
                let mut game_board_clone = (*game_board_state).clone();
                let mut i = 6;

                while i > 0 {
                    i -= 1;
                    if game_board_clone[i][column] == 0 {
                        game_board_clone[i][column] = 1;
                        game_board_state.set(game_board_clone);
                        player_turn.set(false);
                        break;
                    }
                }
            }
        })
    };

    use_effect_with_deps(move |game_board_state_clone| {
        let document = window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("gameBoard").unwrap();
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        // Clear the canvas and fill the background
        context.set_fill_style(&JsValue::from_str("pink"));
        context.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

        // Iterate through the game board state to draw circles
        for (i, row) in game_board_state_clone.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                context.begin_path();
                context.arc(75.0 * (j as f64) + 100.0, 75.0 * (i as f64) + 50.0, 30.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
                // Determine the fill color based on the cell value
                let mut color = JsValue::from_str("white");
                if cell == 1 {
                    color = JsValue::from_str("red");
                } else if cell == 2 {
                    color = JsValue::from_str("yellow");
                }
                context.set_fill_style(&color);
                context.fill();
            }
        }

        let game_board_state_clone = game_board_state_clone.clone();
        let game_board_clone = (*game_board_state_clone).clone();
        let game_board_clone_clone = game_board_clone.clone();
        let player_name = player_name.clone();
        let player_clone = (*player_name).clone();
        let game_is_over = game_is_over.clone();
        let winner_result = check_winner(game_board_clone);
        web_sys::console::log_1(&format!("Player AAAA: {:?}", game_board_clone_clone).into());
        match winner_result {
            1 => {
                game_is_over.set(true);
                winner.set(player_clone);
            },
            2 => {
                game_is_over.set(true);
                winner.set("AlphaPuff".to_string());
            },
            3 => {
                game_is_over.set(true);
                winner.set("Draw".to_string());
            },
            _ => {}
        }

        || ()
    }, game_board_state_clone.clone());

    // use_effect_with_deps(move |player_turn| {
    //     web_sys::console::log_1(&format!("Player: {:?}", player_turn).into());
    //     let player_turn = player_turn.clone();
    //     let player_turn_clone = (*player_turn).clone();
    //     let difficulty_level = difficulty_level.clone();
    //     let difficulty_clone = (*difficulty_level).clone();
    //
    //     if (!player_turn_clone) {
    //         alphaPuff_turn(&game_board_state, &player_turn, difficulty_clone);
    //     }
    //     || ()
    // }, player_turn.clone());

    use_effect_with_deps({
        let player_turn = player_turn.clone();
        let difficulty_level = difficulty_level.clone();

        move |_| {
            web_sys::console::log_1(&format!("Player: {:?}", player_turn).into());
            let player_turn_value = (*player_turn).clone();
            let difficulty_value = (*difficulty_level).clone();
            let game_over = *game_is_over_clone_clone;
            if !player_turn_value && !game_over {
                alphaPuff_turn(&game_board_state, &player_turn, difficulty_value);
            }
            || ()
        }
    }, player_turn.clone());

    html! {
        <div>
        <canvas id="gameBoard" height="480" width="640" style="margin-top:50px" onclick={onClickHandle}></canvas>
        if (*game_is_clone_over).clone() {
            <h4>{format!("{} wins", (*clone_winner).clone())}</h4>
        }
        </div>
    }
}

impl Component for Connect4Computer {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game_started: false,
            player_name: "A".to_string(),
            difficulty_level: "Easy".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GameStarted(player) => {
                self.game_started = true;
                self.player_name = player;
                let document = web_sys::window().unwrap().document().unwrap();
                let diff_selector = document.query_selector("#choose_level")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::HtmlSelectElement>()
                    .unwrap();
                self.difficulty_level = diff_selector.value();
                web_sys::console::log_1(&format!("Level: {:?}", self.difficulty_level).into());
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
                    <option value="Easy" selected=true> {"Easy"} </option>
                    <option value="Hard" selected=false> {"Hard"} </option>
                </select>
            </div>

            if self.game_started {
                <div style="margin-left: 10px; margin-top: 40px">
                    <h4>{format!("New Game: {} Vs AlphaPuff", self.player_name)}</h4>
                    <small>{format!("(Disc Colors: {} - ", self.player_name)}<b style="color: red">{"Red"}</b>{" and AlphaPuff - "}<b style="color: yellow">{"Yellow"}</b>{")"}</small>

                    <GameBoard difficulty_level={self.difficulty_level.clone()} player_name={self.player_name.clone()}/>
                </div>
            }

        </div>
        }
    }
}