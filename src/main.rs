use std::sync::Mutex;
use std::io::Read;
use std::rc::Rc;

use curl::easy::Easy;
use gtk::Button;
use gtk::glib::GString;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;

const APP_ID: &str = "net.caverym.Rokmu";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build);
    app.run();
}

fn build(app: &Application) {
    let ip = Rc::new(Mutex::new(GString::from("")));

    let vbox = gtk::Box::builder().orientation(gtk::Orientation::Vertical).build();

    let hbox = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).build();
    let entry = gtk::Entry::new();
    hbox.append(&entry);
    let entry_button = Button::with_label("Set");
    let clone = ip.clone();
    entry_button.connect_clicked(move |_| {
        let mut i = clone.lock().unwrap();
        *i = entry.text();
        println!("set ip: {}", i);
    });
    hbox.append(&entry_button);
    vbox.append(&hbox);

    let hshbox = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).build();
    let home_button = gtk::Button::with_label("Home");
    home_button.connect_clicked(clone!(@weak ip => move |_| {
        post(SendInput::Home, ip);
    }));
    let select_button = gtk::Button::with_label("Select");
    select_button.connect_clicked(clone!(@weak ip => move |_| {
        post(SendInput::Select, ip);
    }));
    hshbox.append(&home_button);
    hshbox.append(&select_button);
    vbox.append(&hshbox);
    

    let abox = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).build();
    let up_button = gtk::Button::with_label("Up");
    up_button.connect_clicked(clone!(@weak ip => move |_| {
        post(SendInput::Up, ip);
    }));
    let down_button = gtk::Button::with_label("Down");
    down_button.connect_clicked(clone!(@weak ip => move |_| {
        post(SendInput::Down, ip);
    }));
    let left_button = gtk::Button::with_label("Left");
    left_button.connect_clicked(clone!(@weak ip => move |_| {
        post(SendInput::Left, ip);
    }));
    let right_button = gtk::Button::with_label("Right");
    right_button.connect_clicked(clone!(@weak ip => move |_| {
        post(SendInput::Right, ip);
    }));
    abox.append(&up_button);
    abox.append(&down_button);
    abox.append(&left_button);
    abox.append(&right_button);
    vbox.append(&abox);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Rokmu")
        .child(&vbox)
        .build();

    window.present();
}

#[derive(Debug)]
enum SendInput {
    Home,
    Select,
    Up,
    Down,
    Left,
    Right,
}

fn post(input: SendInput, res: Rc<Mutex<GString>>) {
    let ip = res.lock().unwrap();
    println!("Sending {:?} to {}", input, ip);
    let data = format!("{:?}", input);
    let mut bytes = data.as_bytes();

    let mut easy = Easy::new();
    easy.url(&format!("http://{}:8060/keypress/{:?}", ip, input))
        .unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut trans = easy.transfer();
    trans
        .read_function(|buf| Ok(bytes.read(buf).unwrap_or(0)))
        .unwrap();
    trans.perform().unwrap();
}
