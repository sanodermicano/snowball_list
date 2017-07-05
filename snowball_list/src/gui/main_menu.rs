extern crate gtk;

use gui::about;

use gtk::prelude::*;
use gtk::{
    IconSize, Image, Label, Menu, MenuBar, MenuItem, Window,
    WindowPosition, WindowType
};

pub fn main_menu() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);

    window.set_title("Snowball List");
    window.set_position(WindowPosition::Center);
    window.set_size_request(690, 420);

    window.connect_delete_event(|_, _|
        {
            gtk::main_quit();
            Inhibit(false)
        });

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let menu_bar = MenuBar::new();
//file menu

    let file = MenuItem::new_with_label("File");
    let file_menu = Menu::new();
    let new = MenuItem::new_with_label("New");
    let open = MenuItem::new_with_label("Open");
    let save = MenuItem::new_with_label("Save");
    let save_as = MenuItem::new_with_label("Save As");
    let settings = MenuItem::new_with_label("Settings");
    let quit = MenuItem::new_with_label("Quit");

    let new_item = MenuItem::new();
    let new_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let new_label = Label::new(Some("New"));
    new_box.pack_start(&new_label, true, true, 0);

    new_item.add(&new_box);

    file_menu.append(&new);
    file_menu.append(&open);
    file_menu.append(&save);
    file_menu.append(&save_as);
    file_menu.append(&settings);
    file_menu.append(&quit);
    file.set_submenu(Some(&file_menu));
    menu_bar.append(&file);

//task menu

    let task = MenuItem::new_with_label("Task");

    let task_menu = Menu::new();
    let new_task = MenuItem::new_with_label("New Task");
    let edit_task = MenuItem::new_with_label("Edit Task");
    let delete_task = MenuItem::new_with_label("Delete Task");
    let finish_task = MenuItem::new_with_label("Finish Task");
    let show_graph = MenuItem::new_with_label("Show Graph");
    let take_a_break = MenuItem::new_with_label("Take a break");

    let new_item = MenuItem::new();
    let new_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let new_label = Label::new(Some("New"));
    new_box.pack_start(&new_label, true, true, 0);

    new_item.add(&new_box);

    let task_item = MenuItem::new();
    let task_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let task_label = Label::new(Some("Task"));
    task_box.pack_start(&task_label, true, true, 0);
    task_item.add(&task_box);

    task_menu.append(&new_task);
    task_menu.append(&edit_task);
    task_menu.append(&delete_task);
    task_menu.append(&finish_task);
    task_menu.append(&show_graph);
    task_menu.append(&take_a_break);
    task.set_submenu(Some(&task_menu));
    menu_bar.append(&task);

    //About Menu
    let about = MenuItem::new_with_label("About");
    let about_label = MenuItem::new_with_label("About");
    let about_menu = Menu::new();
    about_menu.append(&about_label);
    about.set_submenu(Some(&about_menu));
    menu_bar.append(&about);

    //Operations
    quit.connect_activate(|_|
        {
            gtk::main_quit();
        });

    about_label.connect_activate( move |_|
        {
            about::about();
        });

    v_box.pack_start(&menu_bar, false, false, 0);
    window.add(&v_box);
    window.show_all();

    gtk::main();
}
