extern crate crypto_tcp_chat_client;

use crypto_tcp_chat_client::chat_gui;
use crypto_tcp_chat_client::chat;
use std::thread;
fn main() {

   
    let response=chat::connect_to_server();              
    chat_gui::gui_init(response);

}

