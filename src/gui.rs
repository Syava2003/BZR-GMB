use gtk::prelude::*;
use gtk::{Button, Box, ApplicationWindow};

mod grid;

pub fn build_ui(window: &ApplicationWindow)
{
    //Создание Box 
    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(2)
        .build();

    window.set_child(Some(&hbox));


    // Создание grid
    let grid = gtk::Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(6)
        .margin_bottom(6)
        .row_spacing(2)
        .column_spacing(5)
        .valign(gtk::Align::Fill)
        .vexpand(true)
        .build();

    hbox.append(&grid);


    //Вложенный Box
    let buttonbox = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(15)
        .halign(gtk::Align::End)
        .margin_end(20)
        .margin_bottom(5)
        .build();

    hbox.append(&buttonbox);



    let buttonok = Button::builder()
        .label("       OK       ")
        .build();

    buttonbox.append(&buttonok);

    let buttonclose = Button::builder()
        .action_name("Закрыть")
        .label("  Закрыть  ")
        .build();

    buttonbox.append(&buttonclose);

    buildgrid(&grid, &buttonok, &buttonclose);
}