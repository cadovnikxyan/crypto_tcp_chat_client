extern crate ini;

use std::thread;
use std::io::prelude::*;
use std::net::{TcpListener,TcpStream};
use self::ini::Ini;
use std::collections::VecDeque;

struct server_config{
	host :String,
	
}

fn init() ->server_config{
	let conf = Ini::load_from_file("config.ini").unwrap();
	let section = conf.section(Some("server".to_owned())).unwrap();
	let server : server_config = server_config{host:section.get("host").unwrap().to_string()};
	server
}

fn handler(stream:TcpStream){
	let mut buffer=String::new();
	let mut s:TcpStream=stream;
	s.read_to_string(&mut buffer);
	println!("read: {}",buffer);
	s.write_all(b"success!");
}

pub fn connect_to_server()->String{
	
	let server= init();
	// let mut clients : VecDeque<chat_client> = VecDeque::new();
	let host: &str = &server.host[..];
	println!("connect to host: {}",&server.host);
	  let handler=thread::spawn(move||
	                    {
							let mut buf=String::new();
							{
		                        println!("connected!");
								let mut stream = TcpStream::connect("localhost:3333").unwrap();
								let _ = stream.write_all(b"hello").unwrap();
								// let _ = stream.read_to_string(&mut buf).unwrap();
								
							}
							buf
	                    });
	let responce = handler.join().unwrap();
	println!("{}",responce);
	responce
}