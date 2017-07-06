
pub extern crate gtk;
use gtk::prelude::*;
use gtk::{AboutDialog,Window,Builder};


    pub fn about(window: &Window)
    {
        if gtk::init().is_err()
        {
            println!("Failed to initialize GTK.");
            return;
        }

        let glade_src = include_str!("about.glade");
        let builder = Builder::new_from_string(glade_src);

        let about_window: AboutDialog = builder.get_object("main about").unwrap();

        about_window.connect_delete_event(|_, _|
            {
                gtk::main_quit();
                Inhibit(false)
            });

        about_window.set_transient_for(Some(window));
        about_window.run();
        about_window.destroy();
        gtk::main();
    }
