extern crate cgmath;

use cgmath::prelude::*;
use cgmath::Point2;
use std::net::UdpSocket;
use std::clone::{Clone};
use std::collections::HashMap;
use std::fmt;

fn parse_cmd(msg: String) -> Result<(String, Vec<String>), String> {
    let mut msg_vec: Vec<String> = msg.split_whitespace().map(|s| s.to_string()).collect();
    if msg_vec.len() < 1 {
        Err("Too few args".to_string())
    } else {
        let mut args = Vec::<String>::new();
        while msg_vec.len() > 1 {
            args.push(msg_vec.pop().unwrap());
        }
        let cmd = msg_vec.pop().unwrap().to_string();
        Ok((cmd, args))
    }
}

#[derive(Clone, Debug)]
struct Game {
player_list: HashMap<String, Player>,
}

impl Game {

fn new() -> Game {
    Game { player_list: HashMap::<String, Player>::new() }
}

fn broadcast(&mut self, msg: String, socket: &UdpSocket) {
    for addr in self.player_list.keys() {
        socket.send_to(msg.as_bytes(), addr);
    }
}

fn sender(&mut self, address: String, msg: String , socket: &UdpSocket) {
    socket.send_to(msg.as_bytes(), address);
}

fn send_player_list(&mut self, socket: &UdpSocket) {
    let pl = self.player_list.clone();
    let pl_str : Vec<String> = pl.values().map(|p| format!("{}", p)).collect();
    self.broadcast(format!("update {:?}", pl_str) , socket);
}

fn connect_player(&mut self, address: String, socket: &UdpSocket) {
    let id = self.player_list.len() + 1;
    let player = Player::new(id, Point2::new(100,100));
    self.player_list.insert(address.clone(), player);
    self.sender(address, format!("connect {}", id), socket);
    self.send_player_list(socket);
}

fn update_player(&mut self, address: String, args: Vec<String>, socket: &UdpSocket) {
    match args[1].as_str() {
        "move" => self.player_list.get_mut(&address).unwrap().move_player(args[0].clone()),
        _ => println!("Неверный update"),
    }
    self.send_player_list(socket);
}

fn disconnect_player(&mut self, address: String, socket: &UdpSocket) {
    self.player_list.remove(&address);
    self.send_player_list(socket);
}

pub fn run(&mut self) {
    self.listener("127.0.0.1:34254".to_string());
}

fn msg_handler(&mut self, socket: &UdpSocket, msg: String, src: String) {
    let res = parse_cmd(msg);
    if res.is_ok() {
        let (cmd, args) = res.unwrap();
        println!("{} {:?}", cmd, args);
        match cmd.as_ref() {
            "connect" => self.connect_player(src, socket),
            "update" => self.update_player(src, args, socket),
            "disconnect" => self.disconnect_player(src, socket),
            _ => println!("oi"),
        }
    }
}

fn listener(&mut self, address: String) {
    let socket = UdpSocket::bind(address).expect("couldn't bind to address");
    let mut buf = [0; 64];
    loop {
        let msg = socket.recv_from(&mut buf);
        match msg {
            Ok((m, src)) => {
                let data = Vec::from(&buf[0..m.clone()]);
                match String::from_utf8(data) {
                    Ok(msg_str) => {
                            self.msg_handler(&socket, msg_str, src.to_string());
                            println!("{:?}", self.player_list);
                        },
                    Err(e) => println!("Shit happens {}", e),
                }
            },
            Err(e) => println!("Shit happens {}", e),
        }
    }
}
}

#[derive(Clone, Debug)]
struct Player {
pub id: usize,
pub xy: Point2<i32>,
}

impl Player {
    fn new(id: usize, xy: Point2<i32>) -> Player {
        Player { id, xy }
    }

    pub fn move_player(&mut self, direction: String) {
        match direction.as_str() {
            "up" => self.xy.y -= 1,
            "down" => self.xy.y += 1,
            "left" => self.xy.x -= 1,
            "right" => self.xy.x += 1,
            _ => println!("Неверное направление"),
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}:{}", self.id, self.xy.x, self.xy.y)
    }
}


fn main() {
    let mut game = Game::new();
    game.run();
}
