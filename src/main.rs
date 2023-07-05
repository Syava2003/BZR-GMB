use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
const APP_ID: &str = "com.github.gtk-rs.examples.builder_basics";

mod gui;
mod grid;
mod libs;

fn main() -> glib::ExitCode {
    let application = gtk::Application::new(
        Some(APP_ID),
        Default::default(),
    );
    application.connect_activate(build_window);
    application.run()
}

fn build_window(application: &Application) {
    let window = ApplicationWindow::new(application);
    window.set_title(Some("BZR-GMB"));
    window.set_default_size(800, 600);

    gui::build_ui(&window);

    window.present();
}
