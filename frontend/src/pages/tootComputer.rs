use std::net::ToSocketAddrs;
use reqwest::Client;
use serde_json::json;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{CanvasRenderingContext2d, Event, HtmlCanvasElement, MouseEvent, window};
use yew::{Callback, Component, Context, function_component, Html, html, Properties, TargetCast, use_effect_with_deps, use_state, UseStateHandle};
use crate::pages::c4Computer::alphaPuff_turn;

// 1 for Human T
// 2 for Human O
// 3 for Puff T
// 4 for Puff O
pub struct TOOTComputer {
    game_started: bool,
    player_name: String,
    toro: String,
    difficulty_level: String
}

pub enum Msg_TOOT {
    TOOTGameStarted(String),
    TOOTSelectChange(String),
}

#[derive(Properties, PartialEq)]
pub struct GameBoardProps {
    pub player_name: String,
    pub select_type: String,
    pub difficulty_level: String,
}

fn contains_toot_or_otto(sequence: &str) -> i64 {
    if sequence.contains("TOOT") {
        1
    } else if sequence.contains("OTTO") {
        2
    } else {
        0
    }
}

fn check_pattern_in_board(game_board: Vec<Vec<char>>) -> i64 {
    let cols = game_board[0].len();
    let rows = game_board.len();

    for row in &game_board {
        let sequence: String = row.iter().collect();
        let win_or_not = contains_toot_or_otto(&sequence);
        if win_or_not != 0 {
            return win_or_not;
        }
    }

    for col in 0..cols {
        let column_sequence: String = game_board.iter().map(|row| row[col]).collect();
        let win_or_not = contains_toot_or_otto(&column_sequence);
        if win_or_not != 0 {
            return win_or_not;
        }
    }

    for start_col in 0..=cols-4 {
        let mut diag_sequence = String::new();
        for i in 0..rows.min(cols - start_col) {
            diag_sequence.push(game_board[i][start_col + i]);
        }
        let win_or_not = contains_toot_or_otto(&diag_sequence);
        if win_or_not != 0 {
            return win_or_not;
        }
    }

    for start_col in 3..cols {
        let mut diag_sequence = String::new();
        for i in 0..rows.min(start_col + 1) {
            diag_sequence.push(game_board[i][start_col - i]);
        }
        let win_or_not = contains_toot_or_otto(&diag_sequence);
        if win_or_not != 0 {
            return win_or_not;
        }
    }
    0
}

fn num_2_char(cell: i64) -> char {
    match cell {
        1 | 3 => 'T',
        2 | 4 => 'O',
        _ => '0',
    }
}

fn check_toot_winner(game_board: Vec<Vec<i64>>) -> i64 {
    let char_board: Vec<Vec<char>> = game_board
        .iter()
        .map(|row| row.iter().map(|&cell| num_2_char(cell)).collect())
        .collect();

    let contain_pattern = check_pattern_in_board(char_board.clone());

    let mut nonzero_count= 0;
    if contain_pattern == 0 {
        nonzero_count = char_board.iter().flatten().filter(|&&c| c != '0').count();
        if nonzero_count == 24 {
            return 3
        }
        return 0
    }

    contain_pattern
}

async fn post_game_toot_move(game_board: Vec<Vec<i64>>, difficulty_level: String) -> Result<String, reqwest::Error> {
    web_sys::console::log_1(&format!("Enter post_game_toot_move").into());
    let client = Client::new();
    let res = client.post("http://127.0.0.1:4869/api/puff_toot_step")
        .json(&json!({"game_board": game_board, "difficulty_level": difficulty_level}))
        .send()
        .await?;

    let tootResponse: serde_json::Value = res.json().await?;
    Ok(tootResponse["move_step"].as_str().unwrap_or_default().to_string())
}

pub fn alphaPuff_toot_turn(game_board_state: &UseStateHandle<Vec<Vec<i64>>>, player_turn: &UseStateHandle<bool>, difficulty_level: String) {
    let game_board_state = game_board_state.clone();
    let player_turn = player_turn.clone();
    let player_clone = (*player_turn).clone();

    if !player_clone {
        let game_board_clone = (*game_board_state).clone();
        let mut new_board = game_board_clone.clone();
        spawn_local(async move {
            match post_game_toot_move(game_board_clone, difficulty_level.clone()).await {
                Ok(move_step) => {
                    let split: Vec<&str> = move_step.split(',').collect();
                    if split.len() == 2 {
                        if let Ok(column) = split[0].parse::<i64>() {
                            if let Ok(t_or_o) = split[1].parse::<i64>() {
                                let mut row = 4;
                                while row >= 0 {
                                    row -= 1;
                                    if new_board[row][column as usize] == 0 {
                                        new_board[row][column as usize] = t_or_o;
                                        break;
                                    }
                                }
                                game_board_state.set(new_board);
                                player_turn.set(true);
                                web_sys::console::log_1(&format!("Received column: {}", column).into());
                            }
                        }
                    }
                }
                Err(error) => {
                    web_sys::console::log_1(&format!("HTTP Request failed: {:?}", error).into());
                }
            }
        })
    }
}

#[function_component(TooTGameBoard)]
fn toot_game_board(props: &GameBoardProps) -> Html {
    let game_board_state = use_state(|| vec![vec![0; 6]; 4]);
    let player_turn = use_state(|| true);
    let game_is_over = use_state(|| false);
    let player_name = use_state(|| props.player_name.clone());
    let select_type = use_state(|| props.select_type.clone());
    let winner = use_state(|| "A".to_string());
    let step_type = use_state(|| "T".to_string());
    let difficulty_level = use_state(|| props.difficulty_level.clone());

    // clone variables
    let game_is_clone_over = game_is_over.clone();
    let game_over_copy_2 = game_is_clone_over.clone();
    let clone_winner = winner.clone();
    let game_board_state_clone = game_board_state.clone();
    let player_turn_clone = player_turn.clone();
    let select_type_clone = select_type.clone();

    //onclickevent listener
    let onClickTooTHandle = {
        web_sys::console::log_1(&format!("Player AAAAAAAA: {:?}", select_type.clone()).into());
        let game_board_state = game_board_state.clone();
        let select_type_clone = select_type.clone();
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
            let column = ((x - 100.0) / 90.0).round() as usize;

            // update the state map based on the clicked column
            if column >= 0 && column < 6 {
                let mut game_board_clone = (*game_board_state).clone();
                let mut select_type_clone_value = (*select_type_clone).clone();
                let mut i = 4;
                while i > 0 {
                    i -= 1;
                    if game_board_clone[i][column] == 0 {
                        if (select_type_clone_value == "T") {
                            game_board_clone[i][column] = 1;
                        } else if select_type_clone_value == "O" {
                            game_board_clone[i][column] = 2;
                        }
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
                let x = 90.0 * (j as f64) + 100.0;
                let y = 90.0 * (i as f64) + 50.0;

                context.begin_path();
                context.arc(90.0 * (j as f64) + 100.0, 90.0 * (i as f64) + 50.0, 30.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
                // Determine the fill color based on the cell value
                let (color, text) = match cell {
                    1 => ("red", "T"),
                    2 => ("red", "O"),
                    3 => ("yellow", "T"),
                    4 => ("yellow", "O"),
                    _ => ("white", ""),
                };
                context.set_fill_style(&JsValue::from_str(color));
                context.fill();

                context.set_fill_style(&JsValue::from_str("black"));
                context.set_font("24px sans-serif");
                context.fill_text(text, x - (20.0 / 2.0), y + 8.0).unwrap();
            }
        }

        let game_board_state_clone = game_board_state_clone.clone();
        let game_board_clone = (*game_board_state_clone).clone();
        let player_name = player_name.clone();
        let player_clone = (*player_name).clone();
        let game_is_over = game_is_over.clone();
        let winner_result = check_toot_winner(game_board_clone);
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

    use_effect_with_deps({
        let player_turn = player_turn.clone();
        let difficulty_level = difficulty_level.clone();

        move |_| {
            let player_turn_value = (*player_turn).clone();
            let difficulty_value = (*difficulty_level).clone();
            let game_over = (*game_over_copy_2);
            if !player_turn_value && !game_over {
                alphaPuff_toot_turn(&game_board_state, &player_turn, difficulty_value)
            }
            || ()
        }
    }, player_turn.clone());

    use_effect_with_deps(move |select_type_prop| {
        select_type.set((*select_type_prop).clone());
        || ()
    }, props.select_type.clone());

    html! {
        <div>
        <canvas id="gameBoard" height="390" width="640" style="margin-top:50px" onclick={onClickTooTHandle}></canvas>
        if (*game_is_clone_over).clone() {
            <h4>{format!("{} wins", (*clone_winner).clone())}</h4>
        }
        </div>
    }
}

impl Component for TOOTComputer {
    type Message = Msg_TOOT;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game_started: false,
            player_name: String::from(""),
            toro: String::from("T"),
            difficulty_level: "Easy".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg_TOOT::TOOTGameStarted(player) => {
                self.game_started = true;
                self.player_name = player;
                let document = web_sys::window().unwrap().document().unwrap();
                let diff_selector = document.query_selector("#choose_diff_level")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::HtmlSelectElement>()
                    .unwrap();
                self.difficulty_level = diff_selector.value();
                web_sys::console::log_1(&format!("Level: {:?}", self.difficulty_level).into());
                true
            }
            Msg_TOOT::TOOTSelectChange(selection) => {
                self.toro = selection;
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        unimplemented!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_startGame = ctx.link().callback(|_| {
            let input = gloo_utils::document().get_element_by_id("textbox1").unwrap();
            let input: web_sys::HtmlInputElement = input.dyn_into::<web_sys::HtmlInputElement>().unwrap();
            let name = input.value();
            Msg_TOOT::TOOTGameStarted(name)
        });

        let onSelectChange = ctx.link().callback(|e: Event| {
            let select_element = e.target_dyn_into::<web_sys::HtmlSelectElement>().unwrap();
            Msg_TOOT::TOOTSelectChange(select_element.value())
        });

        html! {
        <div class="main" style="margin-left: 390px; margin-right: 40px">
            <div class="w3-container" id="services" style="margin-top: 75px">
                <h5 class="w3-xxxlarge" style="color: violet"><b>{"Enter Player Names"}</b></h5>
                <hr style="width: 50px; border: 5px solid violet" class="w3-round" />
            </div>
            <div class="col-md-offset-4 col-md-8" style="margin-left: 10px; margin-top: 20px">
                <input id="textbox1" type="text" placeholder="Your Name" />
                <button id="startbutton" class="button" onclick={onclick_startGame}>{ "Start Game" }</button>
            </div>
            <div style="margin-left: 10px; margin-top: 20px">
                <label for="choose_diff_level"> {"Choose difficulty level:"} </label>
                <select id="choose_diff_level" style="margin: 3px">
                    <option value="Easy" selected=true> {"Easy"} </option>
                    <option value="Hard" selected=false> {"Hard"} </option>
                </select>
            </div>
            if self.game_started {
                <div style="margin-left: 10px; margin-top: 40px">
                    <h4>{format!("New Game: {} Vs AlphaPuff", self.player_name)}</h4>
                    <small>{format!("(Winning Combination: {} - ", self.player_name)} <b>{"TOOT"}</b> {"   and    Computer - "} <b>{"OTTO)"}</b></small>
                    <br/>
                    <div style="margin-top: 20px">
                        <label for="choose_level"> {"Choose T or O:"} </label>
                        <select id="choose_level" style="margin: 3px" onchange={onSelectChange}>
                            <option value="T" selected=true> {"T"} </option>
                            <option value="O" selected=false> {"O"} </option>
                        </select>
                    </div>
                    <TooTGameBoard player_name={self.player_name.clone()} select_type={self.toro.clone()} difficulty_level={self.difficulty_level.clone()}/>
                </div>
            }
        </div>
        }
    }
}