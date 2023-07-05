use gtk::prelude::*;
use gtk::{Button, Grid, CheckButton};

use crate::libs;

pub fn buildgrid(grid: &Grid, buttonok: &Button, buttonclose: &Button)
{
    let grid1 = libs::FirstGrid {
        select: "Выбор",
        logo: "Лого",
        name: "Название",
        option: "Описание",
        author: "Автор",
    };

    grid1.summarize(&grid);


    let mut programm: Vec<CheckButton> = vec![];
    let mut cmd: Vec<&str> = vec![];

    let grid2 = libs::OtherGrid {
        logo: "cpu-x",
        name: "CPU-X",
        option: 
        "\tCPU-X - это утилита для вывода информации о процессоре, материнской памяти, 
        оперативной памяти, графического процессора и о самой операционной системе. 
        Также имеет встроенный бенчмарк процессора. Является почти полным аналогом 
        CPU-Z в среде Windows",
        author: "Syava",
        row_num: 2,
    };

    let grid2cpux = grid2.summarize(&grid);

    programm.push(grid2cpux);
    cmd.push("sudo apt-get install cpux");

    let grid3 = libs::OtherGrid {
        logo: "krita",
        name: "Krita",
        option: "\tутилита для рисования",
        author: "Syava",
        row_num: 3,
    };

    let grid3krita = grid3.summarize(&grid);


    programm.push(grid3krita);
    cmd.push("sudo apt-get install krita");

    buttonok.connect_clicked(move |_| libs::clickedbutton(programm.clone(), cmd.clone()));
    
}
