extern crate crypto_tcp_chat_client;

use crypto_tcp_chat_client::chat_gui;
use crypto_tcp_chat_client::chat_mio_example;
use std::thread;
fn main() {

   
    // let response=chat::connect_to_server();              
    chat_mio_example::run("10.0.0.75:3333".parse().unwrap());
    chat_gui::gui_init("response".to_string());

}

