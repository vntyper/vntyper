// Copyright 2016 Do Duy.
// Licensed under the MIT license, see the LICENSE file or
// <http://opensource.org/licenses/MIT>

extern crate gtk;
extern crate vntyper;

use vntyper::input;
use vntyper::input_method;

use gtk::prelude::*;
use gtk::Window;
use gtk::TextView;
use gtk::TextBuffer;
use gtk::TextIter;

trait Buffer {
    fn get_insert_iter(&self) -> Option<TextIter>;
    fn complete(&self, c: char) -> Inhibit ;
}

macro_rules! gtk_try {
    ( $x:expr ) => {
        {
            let tmp = $x;
            if $x.is_none() { return Inhibit(false) }
            tmp.unwrap()
        }
    };
}

impl Buffer for TextBuffer {
    fn get_insert_iter(&self) -> Option<TextIter> {
        self.get_insert().map(|x| self.get_iter_at_mark(&x))
    }
    fn complete(&self, c: char) -> Inhibit {
        let mut end_iter = gtk_try!(self.get_insert_iter());

        let mut start_iter = end_iter.clone();
        start_iter.backward_chars(15);

        let text = gtk_try!(self.get_text(&start_iter, &end_iter, false));
        let vntyper = input::Input::new(text, c, input_method::InputMethod::telex());
        let output = vntyper.output();
        println!("{:?}", output);
        let mut set_text = |s: &str| {
            self.delete(&mut start_iter, &mut end_iter);
            self.insert(&mut start_iter, s);
        };
        match output {
            Err(s) => {
                set_text(&s);
                Inhibit(false)
            },
            Ok(s) => {
                set_text(&s);
                Inhibit(true)
            },
        }
    }
}

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

    text_view.connect_key_press_event(|widget, ev| {
        match std::char::from_u32(ev.get_keyval()) {
            Some(key) => match widget.get_buffer() {
                Some(buffer) => buffer.complete(key),
                None => Inhibit(false),
            },
            None => Inhibit(false),
        }
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    gtk::main();
}
