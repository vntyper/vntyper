extern crate gtk;
use gtk::prelude::*;
use gtk::Window;
use gtk::TextView;

fn main() {
    if gtk::init().is_err() {
        panic!();
    }
    let window = Window::new(gtk::WindowType::Toplevel);
    let text_view = TextView::new();
    window.set_default_size(400, 400);
    window.add(&text_view);
    window.set_title("Vietnamese Input");
    window.show_all();

    text_view.connect_key_press_event(|w, ev| {
        println!("{}", ev.get_keyval() as u8 as char);
        Inhibit(false)
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    gtk::main();
}
