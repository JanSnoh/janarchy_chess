
//use crate::backend::moves::Field;


use std::{borrow::Cow, thread::{Thread, self}, collections::HashMap, fs::read_dir, io};

use janarchy_chess::{GameState, pieces::{self, Piece, PieceColor}, Field, Move};
use macroquad::{prelude::*, ui::{self, UiContent, root_ui}, rand::ChooseRandom, experimental::collections::storage};

//static mut FIELD_SIZE: f32 = 8.0;

#[derive(Clone, Copy, Debug)]
struct LayoutData{
    x: f32,
    y: f32,
    field_size: f32
}
impl LayoutData {
    fn update(self) -> Self{
        let x = 0.0;
        let y = 0.0;
        let field_size = f32::min(screen_height(), screen_width())/8.0;
        LayoutData { x, y ,field_size }
    }
    fn new() -> Self{
        LayoutData { x: 0.0, y: 0.0, field_size: 0.0 }
    }
}

#[macroquad::main("Anarchy")]
async fn main() {
    let mut game_state = GameState::default();
    let mut lifted: Option<Field> = None;
    //game_state.print(Some(pieces::PieceColor::Black));

    //let size = f32::min(screen_height(), screen_width());
    let layout = LayoutData::new().update();
    //let input_handle = thread::

    let mut texture_map = HashMap::new();
    //texture_map.insert("rook".to_string(), texture);
    load_piece_textures(&mut texture_map).await.unwrap();
    build_textures_atlas();

    loop{
        clear_background(GRAY);
        let layout = layout.update();
        let draw_conf = DrawTextureParams{ 
            dest_size: Some(Vec2 { x: layout.field_size, y: layout.field_size }),
            .. Default::default()};

        draw_board(&game_state ,layout, &texture_map, &draw_conf, lifted);

        let cont = UiContent::Label(Cow::from("Hey"));
        ui::widgets::Button::new(cont);
        if root_ui().button(None, "Button time") {
            if let Some(x) = game_state.possible_moves(pieces::PieceColor::White).choose(){
                lifted = None;
                game_state.apply_move(x.clone());
            }
        }
        input_handler(layout, &mut lifted, &mut game_state);

        //dbg!(get_fps());
        next_frame().await;
    }
}

///X Y pos and side length plus fs 
fn draw_board(gs: &GameState, layout: LayoutData, texture_map: &HashMap<String, Texture2D>, 
    draw_conf: &DrawTextureParams, lifted: Option<(Field)>) {

    let LayoutData{x: x_offset, y: y_offset, field_size} = layout;
    let draw_conf = DrawTextureParams{ 
        dest_size: Some(Vec2 { x: layout.field_size, y: layout.field_size }),
        .. Default::default()};
    //draw hovered piece

    //if let Some((_, hov_piece)) = lifted {
    //    let texture = texture_map.get(&piece_string(hov_piece)).unwrap();
    //    let mouse = mouse_position();
    //    draw_texture_ex(texture.clone(), mouse.0, mouse.1, WHITE, draw_conf);
    //}

    for field in GameState::iter_squares() {
        let (x, y) = (x_offset+field.0 as f32*field_size, y_offset+field.1 as f32*field_size);
        draw_rectangle(
            x,
            y, 
            field_size, 
            field_size, 
            field_to_color(field));
        //debug remove on release
        draw_text(&Piece::to_char(&gs[field]).to_string(),
            x_offset+ 0.5*field_size + x, 
            y_offset+ 0.5*field_size + y, 
            40.0, 
            GOLD);
            
        if let Some(piece) = gs[field] {
            let texture = *texture_map.get(&piece_string(piece)).unwrap();
            if !matches!(lifted, Some(lift_field) if lift_field == field) {
                draw_texture_ex(texture, x, y, WHITE, draw_conf.clone());
            }
        }
    }
    if let Some(lift_field) = lifted {
        let (mouse_x, mouse_y) = mouse_position();
        let lift_texture = *texture_map.get(&piece_string(gs[lift_field].unwrap())).unwrap();
        draw_texture_ex(lift_texture, mouse_x-field_size/2.0, mouse_y-field_size/2.0, WHITE, draw_conf);
    }
}


fn pos_to_field(pos: Vec2, layout: LayoutData) -> Option<Field>{
    let pos = pos/layout.field_size;
    if pos.max_element() > 8.0 {return None}
    Some(Field(pos.x as usize,pos.y as usize))
}

///My first time dealing with GPU rendering. Forgive me for this code
async fn load_piece_textures(container: &mut HashMap<String, Texture2D>) -> anyhow::Result<()>{
    for entry in read_dir("res/pieces")?{
        let entry = entry?;
        let name: String = entry.file_name().to_str().ok_or(anyhow::Error::msg("placeholder error"))?.to_string();
        dbg!(entry.path());
        container.insert(name, load_texture(entry.path().to_str().unwrap()).await?);
    }

    Ok(())
}

fn field_to_color(field: Field) -> Color {  if (field.0+field.1)%2 == 0 { WHITE } else { BROWN } }


fn piece_string(piece: Piece) -> String{
    let out = String::new();
    let type_name = match piece.piece_type {
        pieces::PieceType::King => "king",
        pieces::PieceType::Queen => "queen",
        pieces::PieceType::Bishop => "bishop",
        pieces::PieceType::Knight => "knight",
        pieces::PieceType::Rook => "rook",
        pieces::PieceType::Pawn => "pawn",
        _ => unreachable!()
    };
    let color_name = match piece.color {
        pieces::PieceColor::Black => "black",
        pieces::PieceColor::White => "white",
    };
    format!("{type_name}_{color_name}.png")
}

fn input_handler(layout: LayoutData, lifted: &mut Option<Field>, game_state: &mut GameState){
    if let Some(hov_field) = pos_to_field(mouse_position().into(), layout) {
        if is_mouse_button_pressed(MouseButton::Left) {
            dbg!(pos_to_field(mouse_position().into(), layout));
            dbg!(&lifted);
            if let Some((lift_pos)) = lifted.clone() {
                let potential_move = Move::from_squares(lift_pos, hov_field);
                if game_state.possible_moves(PieceColor::White).contains(&potential_move){
                    dbg!(potential_move.clone());
                    game_state.apply_move(potential_move);
                    *lifted = None;
                }
            } else if let Some(_) = game_state[hov_field] {
                *lifted = Some((hov_field));
            }
            
        }
    }
    if is_mouse_button_pressed(MouseButton::Right){
        *lifted = None;
    }
}