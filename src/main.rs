use gtk4 as gtk;
use gdk4 as gdk;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
// use gtk::prelude::*;
use libadwaita::prelude::*;

use curl::easy::Easy;
use gtk::glib::clone;
use gtk::glib::GString;
use gtk::ApplicationWindow;
use gtk::Button;
use libadwaita::Application;

const APP_ID: &str = "net.caverym.Rokmu";

macro_rules! post {
    ($sb:expr, $ip:expr) => {
        if let Err(e) = post($sb, $ip) {
            eprintln!("error: {}", e);
        }
    };
}

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build);
    app.run();
}

fn build(app: &Application) {
    let ip = Arc::new(Mutex::new(String::new()));

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .baseline_position(gtk::BaselinePosition::Center)
        .margin_top(2)
        .build();

    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .baseline_position(gtk::BaselinePosition::Center)
        .margin_top(4)
        .margin_bottom(4)
        .build();
    let entry = gtk::Entry::builder().margin_start(2).margin_end(2).build();
    hbox.append(&entry);
    let entry_button = Button::builder()
        .label("Connect")
        .margin_start(2)
        .margin_end(2)
        .build();
    let clone = ip.clone();
    entry_button.connect_clicked(move |_| {
        let text = entry.text();
        let t = text.to_string();
        let nclone = clone.clone();
        thread::spawn(move || {
            if connection_test(&t) {
                let mut i = nclone.lock().unwrap();
                *i = t;
                println!("set ip: {}", i);
            } else {
                eprintln!("failed to connect to {}", t);
            }
        });
    });
    hbox.append(&entry_button);
    // vbox.append(&hbox);

    let homebackbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .baseline_position(gtk::BaselinePosition::Center)
        .homogeneous(true)
        .build();
    let home_button = gtk::Button::builder()
        .label("Home")
        .margin_start(2)
        .margin_end(2)
        .build();
    home_button.connect_clicked(clone!(@weak ip => move |_| {
        post!(SendInput::Home, ip);
    }));
    let back_button = gtk::Button::builder()
        .label("Back")
        .margin_start(2)
        .margin_end(2)
        .build();
    back_button.connect_clicked(clone!(@weak ip => move |_| {
        post!(SendInput::Back, ip);
    }));
    homebackbox.append(&home_button);
    homebackbox.append(&back_button);
    vbox.append(&homebackbox);

    let ulordbox = gtk::Grid::builder()
        .orientation(gtk::Orientation::Vertical)
        .row_homogeneous(true)
        .column_homogeneous(true)
        .margin_top(4)
        .margin_bottom(2)
        .build();

    let up_button = gtk::Button::builder()
        .label("Up")
        .margin_start(2)
        .margin_end(2)
        .build();
    up_button.connect_clicked(clone!(@weak ip => move |_| {
        post!(SendInput::Up, ip);
    }));
    let down_button = gtk::Button::builder()
    .label("Down")
    .margin_start(2)
    .margin_end(2)
    .build();
    down_button.connect_clicked(clone!(@weak ip => move |_| {
        post!(SendInput::Down, ip);
    }));
    let left_button = gtk::Button::builder()
    .label("Left")
    .margin_start(2)
    .margin_end(2)
    .build();
    left_button.connect_clicked(clone!(@weak ip => move |_| {
        post!(SendInput::Left, ip);
    }));
    let right_button = gtk::Button::builder()
    .label("Right")
    .margin_start(2)
    .margin_end(2)
    .build();
    right_button.connect_clicked(clone!(@weak ip => move |_| {
        post!(SendInput::Right, ip);
    }));
    let ok_button = gtk::Button::builder()
    .label("OK")
    .margin_start(2)
    .margin_end(2)
    .build();
    ok_button.connect_clicked(clone!(@weak ip => move |_| {
        post!(SendInput::Select, ip);
    }));
    ulordbox.attach(&up_button, 1, 0, 1, 1);
    ulordbox.attach(&down_button, 1, 2, 1, 1);
    ulordbox.attach(&left_button, 0, 1, 1, 1);
    ulordbox.attach(&right_button, 2, 1, 1, 1);
    ulordbox.attach(&ok_button, 1, 1, 1, 1);
    vbox.append(&ulordbox);

    let volmbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .homogeneous(true)
        .margin_top(2)
        .margin_bottom(4)
        .margin_start(2)
        .margin_end(2)
        .build();
    let volbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .homogeneous(true)
        .build();
    let vol_up_button = gtk::Button::with_label("Volume Up");
    vol_up_button.connect_clicked(clone!(@weak ip => move |_| {
        post!(SendInput::VolumeUp, ip);
    }));
    let vol_down_button = gtk::Button::with_label("Volume Down");
    vol_down_button.connect_clicked(clone!(@weak ip => move |_| {
        post!(SendInput::VolumeDown, ip);
    }));
    volbox.append(&vol_up_button);
    volbox.append(&vol_down_button);
    volmbox.append(&volbox);

    let mute_button = gtk::Button::with_label("Mute");
    mute_button.connect_clicked(clone!(@weak ip => move |_| {
        post!(SendInput::VolumeMute, ip);
    }));

    volmbox.append(&mute_button);
    vbox.append(&volmbox);

    let titlebar = libadwaita::HeaderBar::builder().title_widget(&hbox).build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Rokmu")
        .default_width(266)
        .default_height(251)
        .resizable(false)
        .titlebar(&titlebar)
        .child(&vbox)
        .build();

    window.present();
}

#[derive(Debug)]
enum SendInput {
    Home,
    Select,
    Back,
    Up,
    Down,
    Left,
    Right,
    VolumeUp,
    VolumeDown,
    VolumeMute,
}

fn post(input: SendInput, res: Arc<Mutex<String>>) -> Result<(), Box<dyn std::error::Error>> {
    let ip = res.lock().unwrap();
    println!("Sending {:?} to {}", input, ip);
    let data = format!("{:?}", input);
    let mut bytes = data.as_bytes();

    let mut easy = Easy::new();
    easy.url(&format!("http://{}:8060/keypress/{:?}", ip, input))?;
    easy.post(true)?;
    easy.post_field_size(data.len() as u64)?;

    let mut trans = easy.transfer();
    trans.read_function(|buf| Ok(bytes.read(buf).unwrap_or(0)))?;
    trans.perform()?;
    Ok(())
}

fn connection_test(ip: &str) -> bool {
    let ip = Arc::new(Mutex::new(ip.to_owned()));
    let one = post(SendInput::VolumeMute, ip.clone());
    let two = post(SendInput::VolumeMute, ip.clone());

    one.is_ok() && two.is_ok()
}
