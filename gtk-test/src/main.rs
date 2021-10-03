use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label};
use gtk::glib::clone;
use std::{
    rc::Rc,
    cell::Cell
};

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk.example")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    let builder = gtk::Builder::from_string(include_str!("ui/main.cmb.ui"));

    macro_rules! build {
        ($name:literal) => {
            builder.object($name).expect(concat!("Couldn't get ", $name, " from builder"))
        }
    }

    let window: ApplicationWindow = build!("main_window");

    window.set_application(Some(app));

    let incr_button: Button = build!("incr_button");

    let decr_button: Button = build!("decr_button");

    let label: Label = build!("counter_label");

    let counter = Rc::new(Cell::new(0 as u8));

    incr_button.connect_clicked(clone!(@strong counter, @strong label =>
            move |_| {
                counter.set(counter.get().wrapping_add(1));
                label.set_text(&counter.get().to_string());
            }
    ));

    decr_button.connect_clicked(clone!(@strong counter, @strong label =>
            move |_| {
                counter.set(counter.get().wrapping_sub(1));
                label.set_text(&counter.get().to_string());
            }
    ));



    // Present window to the user
    window.present();
}

