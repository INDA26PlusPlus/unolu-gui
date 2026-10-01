
pub mod cnet {

use std::{io::{BufRead, BufReader, Read, Write}, net::{TcpListener, TcpStream}};

use viggoskj_chess_lib::*;

pub struct ChessGameConnection {
    stream: TcpStream,
    listener: Option<TcpListener>,
}

impl Drop for ChessGameConnection {
    fn drop(&mut self) {
        self.stream.shutdown(std::net::Shutdown::Both);
    }
}

pub const HOST_ADDR: &str = "127.0.0.1:6767";

#[derive(Eq, PartialEq, Debug)]
pub enum Resp {
    OK = 1,
    REJECT = 2,
    CHECKMATE = 3,
    STALEMATE = 4,
}

impl ToString for Resp {
    fn to_string(&self) -> String {
        return String::from(match self {
            Resp::OK => "OK",
            Resp::REJECT => "REJECT",
            Resp::CHECKMATE=> "CHECKMATE",
            Resp::STALEMATE=> "STALEMATE",
        });
    }
}


impl ChessGameConnection {
    pub fn make_host(opponent_color: Color) -> ChessGameConnection {
        let listener = match TcpListener::bind(HOST_ADDR) {
            Ok(l) => l,
            Err(_) => panic!("Could not bind"),
        };
        let mut stream = match listener.accept() {
            Ok((tcp_stream, sock_addr)) => {
                println!("Connected: {}", sock_addr);
                tcp_stream
            }, 
            Err(_) => panic!("Could not accept"),
        };
        stream.write(match opponent_color {
            Color::Black => "B\n".as_bytes(),
            Color::White => "W\n".as_bytes(),
        }).unwrap();
        stream.set_read_timeout(None).unwrap();
        return ChessGameConnection { stream, listener: Some(listener) };
    }
    pub fn make_client(addr: &str) -> (ChessGameConnection, Color) {
        let mut stream = match TcpStream::connect(addr) {
            Ok(tcp_stream) => tcp_stream,
            Err(_) => panic!("Could not accept"),
        };
        let mut buf: [u8; 2] = [0; 2];
        if stream.read_exact(&mut buf).is_err() {
            panic!("Could not get color");
        }
        let color = match buf[0] as char {
            'B' => Color::Black,
            'W' => Color::White,
            _ => panic!("Invalid color from host"),
        };
        stream.set_read_timeout(None).unwrap();
        return (ChessGameConnection { stream, listener: None }, color);
    }
}

pub fn get_move(cgc: &mut ChessGameConnection) -> Move {
    let mut move_str = String::new();
    let mut bufr = BufReader::new(&mut cgc.stream);
    bufr.read_line(&mut move_str).unwrap();
    let mut opp_move: Move; 
    let pstr = move_str.split_at(4).0.to_ascii_lowercase();
    opp_move = parse_move(pstr.as_str()).unwrap();
    if move_str.chars().nth(4).unwrap() != '-' {
        let p = match move_str.chars().nth(4).unwrap() {
            'R' => PieceType::Rook, 
            'N' => PieceType::Knight, 
            'B' => PieceType::Bishop, 
            'Q' => PieceType::Queen, 
            _ => panic!("\"{}\" is an invalid piece type", move_str.chars().nth(4).unwrap()),
        };
        opp_move = Move::Advanced {
            chess_move: AdvancedMove::Promotion {
            piece_type: p,
            basic_move: BasicMove {
                piece_square: match opp_move { Move::Basic { chess_move } => chess_move.piece_square, _ => panic!("What happends?")},
                target_square: match opp_move { Move::Basic { chess_move } => chess_move.target_square, _ => panic!("What happends?")},
                }
            }
        };
    }
    return opp_move;
}


pub fn send_response(cgc: &mut ChessGameConnection, r: Resp) {
    let mut s = r.to_string();
    s.push_str("\n");
    cgc.stream.write(s.as_bytes()).unwrap();
}

pub fn get_response(cgc: &mut ChessGameConnection) -> Resp {
    let mut r = BufReader::new(&mut cgc.stream);
    let mut response: String = String::new();
    r.read_line(&mut response);
    if response == "REJECT\n" {
        return Resp::REJECT;
    } else if response == "OK\n" {
        return Resp::OK;
    } else if response == "STALEMATE\n" {
        return Resp::STALEMATE;
    } else if response == "CHECKMATE\n" {
        return Resp::CHECKMATE;
    } else  {
        panic!("Invalid response!");
    }
}

pub fn send_move(cgc: &mut ChessGameConnection, m: &viggoskj_chess_lib::Move, g: &viggoskj_chess_lib::Game) -> Resp {
    let mut p: u8 = '-' as u8;
    let start: Square;
    let stop: Square;
    match m {
        Move::Advanced { chess_move } => match chess_move {
            AdvancedMove::Promotion { piece_type, basic_move } => {
                start = basic_move.piece_square;
                stop = basic_move.target_square;
                p = piece_type.to_char().to_ascii_uppercase() as u8;
            },
            AdvancedMove::EnPessant { basic_move } => {
                start = basic_move.piece_square;
                stop = basic_move.target_square;
            },
            _ => panic!("Unknown Advanced Move!"),
        },
        Move::Basic { chess_move } => {
            start = chess_move.piece_square;
            stop = chess_move.target_square;
        }
    }
    let mut s = String::new();
    s.push_str(&((('A' as u32 + start.col) as u8) as char).to_string());
    s.push_str(&((('1' as u32 + start.row) as u8) as char).to_string());
    s.push_str(&((('A' as u32 + stop.col) as u8) as char).to_string());
    s.push_str(&((('1' as u32 + stop.row) as u8) as char).to_string());
    s.push_str(&(p as char).to_string());
    for c in g.board.to_string().replace("-", " ").chars() {
        if c != '\n' {
            s.push_str(&c.to_string());
        }
    }
    s.push_str(&'\n'.to_string());
    cgc.stream.write(&s.into_bytes());
    let resp = get_response(cgc);
    return resp;
}

}
