
use macroquad::prelude::*;
use viggoskj_chess_lib::*;


struct ChessGame {
    game: game::Game,
}

impl ChessGame {
    fn new() -> ChessGame {
        let g: game::Game = create_game();
        return ChessGame {
            game: g,
        }
    }
}

fn draw_board(x_org: u32, y_org: u32, square_size: u32) {
    let mut is_black = false;
    for i in 0..8 {
       for j in 0..8 {
            draw_rectangle((x_org+square_size*i) as f32, (y_org+square_size*j) as f32, square_size as f32, square_size as f32,
                if is_black {macroquad::prelude::Color::from_rgba(51, 110, 65, 255)} else {WHITE},
            );
           is_black = !is_black;
       }
       is_black = !is_black;
   }
}

struct PieceTextures {
    bpawn: Texture2D,
    wpawn: Texture2D,
    brook: Texture2D,
    wrook: Texture2D,
    bknight: Texture2D,
    wknight: Texture2D,
    bbishop: Texture2D,
    wbishop: Texture2D,
    bqueen: Texture2D,
    wqueen: Texture2D,
    bking: Texture2D,
    wking: Texture2D,
}

impl PieceTextures {
    async fn new() -> PieceTextures {
        return PieceTextures {
            bpawn: load_texture("src/pieces/Chess_pdt60.png").await.unwrap(),
            wpawn: load_texture("src/pieces/Chess_plt60.png").await.unwrap(),
            brook: load_texture("src/pieces/Chess_rdt60.png").await.unwrap(),
            wrook: load_texture("src/pieces/Chess_rlt60.png").await.unwrap(),
            bknight: load_texture("src/pieces/Chess_ndt60.png").await.unwrap(),
            wknight: load_texture("src/pieces/Chess_nlt60.png").await.unwrap(),
            bbishop: load_texture("src/pieces/Chess_bdt60.png").await.unwrap(),
            wbishop: load_texture("src/pieces/Chess_blt60.png").await.unwrap(),
            bqueen: load_texture("src/pieces/Chess_qdt60.png").await.unwrap(),
            wqueen: load_texture("src/pieces/Chess_qlt60.png").await.unwrap(),
            bking: load_texture("src/pieces/Chess_kdt60.png").await.unwrap(),
            wking: load_texture("src/pieces/Chess_klt60.png").await.unwrap(),
        };
    }

    fn get_texture(&self, p: &Piece) -> &Texture2D {
        return match p.piece_color {
            viggoskj_chess_lib::Color::Black => match p.piece_type {
                PieceType::Pawn => &self.bpawn,
                PieceType::Rook => &self.brook,
                PieceType::Knight => &self.bknight,
                PieceType::Bishop => &self.bbishop,
                PieceType::Queen => &self.bqueen,
                PieceType::King => &self.bking,
                
            },
            viggoskj_chess_lib::Color::White => match p.piece_type {
                PieceType::Pawn => &self.wpawn,
                PieceType::Rook => &self.wrook,
                PieceType::Knight => &self.wknight,
                PieceType::Bishop => &self.wbishop,
                PieceType::Queen => &self.wqueen,
                PieceType::King => &self.wking,
                
            },
        };
    }
}

fn draw_pieces(cg: &ChessGame, pt: &PieceTextures, x_org: u32, y_org: u32, square_size: u32) {
    for i in 0..8 {
       for j in 0..8 {
           let some_piece = cg.game.board.get_pice(i, j);
           if some_piece.is_some() {
               draw_texture(pt.get_texture(&some_piece.unwrap()), (x_org+square_size*j) as f32, (y_org+square_size*i) as f32, WHITE);

                   /*
               draw_text(
                   some_piece.unwrap().to_char().to_string(),
                   (x_org+square_size/2+square_size*j) as f32,
                   (y_org+square_size/2+square_size*i) as f32,
                   60.0,
                   if some_piece.unwrap().piece_color == viggoskj_chess_lib::Color::Black {BLACK} else {WHITE},
               );
            */
           }
       }
   }
}




#[macroquad::main("Tjackis (Nu med grafik!!!!)")]
async fn main() {
    let mut chess_game = ChessGame::new();
    let ptexs = PieceTextures::new().await;
    let select_pos: (u32, u32) = (0, 0);
    const x_origin: u32 = 100;
    const y_origin: u32 = 100;
    const sq_sz: u32 = 60;
    const x_max: u32 = x_origin + sq_sz*8;
    const y_max: u32 = y_origin + sq_sz*8;
    let mut turn = viggoskj_chess_lib::Color::White;
    let mut selecting = false;
    let mut moves: Vec<BasicMove> = Vec::new();
    let mut sel_piece = (0, 0);
    let mut stop = false;
    loop {
        if stop {
            continue;
        }
        clear_background(GRAY);
        if is_mouse_button_pressed(MouseButton::Left) {
            let (xf,yf) = mouse_position();
            let x = xf as u32;
            let y = yf as u32;
            if x < x_max && x > x_origin && y < y_max && y > y_origin {
                let x_b = (x-x_origin) / sq_sz;
                let y_b = (y-y_origin) / sq_sz;
                let some_p = chess_game.game.board.get_pice(y_b, x_b);
                if some_p.is_some() {
                    if some_p.unwrap().piece_color == turn {
                        moves = Vec::new();
                        if !selecting {
                            selecting = true;
                            sel_piece = (x_b, y_b);
                            for m in possible_legal_moves(&chess_game.game) {
                                if m.piece_square.row == y_b && m.piece_square.col == x_b {
                                    moves.push(m);
                                }
                            }
                        } else {
                            selecting = false;
                        }
                    }
                }
                if selecting {
                    let mut valid = false;
                    let mut valid_move: Option<BasicMove> = None;
                    for m in &moves {
                        if m.target_square.row == y_b && m.target_square.col == x_b {
                            valid = true;
                            valid_move = Some(m.clone());
                            break;
                        }
                    }
                    if valid {
                        match play_move(&chess_game.game, Move::Basic { chess_move: valid_move.unwrap() }) {
                            Ok(g) => {
                                chess_game.game = g;
                                selecting = false;
                                moves = Vec::new();
                                turn = match turn {
                                    viggoskj_chess_lib::Color::Black => viggoskj_chess_lib::Color::White, 
                                    viggoskj_chess_lib::Color::White => viggoskj_chess_lib::Color::Black, 
                                }
                            },
                            Err(_) => panic!("FUCK!!!"),
                        }
                    }
                }

            }  
        }
        draw_board(x_origin,y_origin,sq_sz);
        if selecting {
            draw_rectangle((sel_piece.0*sq_sz+x_origin) as f32, (sel_piece.1*sq_sz+y_origin) as f32, sq_sz as f32, sq_sz as f32, RED);
        }
        draw_pieces(&chess_game, &ptexs, x_origin, y_origin, sq_sz);
        for m in &moves {
            draw_circle((x_origin+m.target_square.col*sq_sz+sq_sz/2) as f32, (y_origin+m.target_square.row*sq_sz+sq_sz/2) as f32, 5.0, RED);
        }
        let mut state_str = match turn {
            viggoskj_chess_lib::Color::White => "Turn: White",
            viggoskj_chess_lib::Color::Black => "Turn: Black"
        }.to_string();

        if is_check(&chess_game.game) {
            match get_check_state(&chess_game.game).unwrap() {
                CheckState::Stalemate => {
                    draw_text(
                        "stalemate",
                        (x_origin+sq_sz*2) as f32,
                        (y_origin+sq_sz*4) as f32,
                        80.0,
                        RED
                    );
                    stop = true;
                },
                CheckState::Checkmate => {
                    draw_text(
                    match turn {
                            viggoskj_chess_lib::Color::White => "Black Wins!",
                            viggoskj_chess_lib::Color::Black => "White wins!",
                        },
                        (x_origin+sq_sz*1) as f32,
                        (y_origin+sq_sz*4) as f32,
                        80.0,
                        RED
                    );
                    stop = true;
                }
                CheckState::Check => {
                    state_str.push_str(" (Check)");
                }
            }
        }

        draw_text(
            state_str,
            30.0,
            30.0,
            40.0,
            BLACK
        );
        next_frame().await;

    };
}
/*
play_move
get_check_state
is_check
select_playing_board
parse_move
parse_square
*/

