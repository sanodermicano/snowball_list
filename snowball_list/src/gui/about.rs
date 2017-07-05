
pub extern crate gtk;
use gtk::prelude::*;
use gtk::{Window,Builder};


    pub fn about()
    {
        if gtk::init().is_err()
        {
            println!("Failed to initialize GTK.");
            return;
        }

        let glade_src = include_str!("about.glade");
        let builder = Builder::new_from_string(glade_src);

        let window: Window = builder.get_object("main about").unwrap();

        window.connect_delete_event(|_, _|
            {
                gtk::main_quit();
                Inhibit(false)
            });

        window.show_all();

        gtk::main();
    }
