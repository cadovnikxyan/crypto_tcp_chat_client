extern crate gtk;
use gtk::prelude::*;
use gtk::{Button,Label, Window, WindowType};


fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(250, 600);
    let button = Button::new_with_label("Click me!");
    window.add(&button);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        println!("Clicked!");
    });

    let second_window= Window::new(WindowType::Toplevel);
    second_window.set_title("Second Window!");
    second_window.set_default_size(200,600);
    let label= Label::new_with_mnemonic(Some("label"));
    second_window.add(&label);
    second_window.show_all();
    gtk::main();
}

