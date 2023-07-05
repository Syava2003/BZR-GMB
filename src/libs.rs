use gtk::prelude::*;
use gtk::{Grid, CheckButton, Label, Image};

pub struct FirstGrid<'a> {
    pub select: &'a str,
    pub logo: &'a str,
    pub name: &'a str,
    pub option: &'a str,
    pub author: &'a str,
}

impl<'a> FirstGrid<'a>{
    pub fn summarize(&self, grid: &Grid) {
        let select = Label::new(Some(&self.select));
        let logo = Label::new(Some(&self.logo));
        let name = Label::new(Some(&self.name));
        let option = Label::new(Some(&self.option));
        let author = Label::new(Some(&self.author));

        grid.attach(&select, 0, 0, 1, 1);
        grid.attach(&logo, 1, 0, 1, 1);
        grid.attach(&name, 2, 0, 1, 1);
        grid.attach(&option, 3, 0, 1, 1);
        grid.attach(&author, 4, 0, 1, 1);
        
    }
}

pub struct OtherGrid<'a> {
    pub logo: &'a str,
    pub name: &'a str,
    pub option: &'a str,
    pub author: &'a str,
    pub row_num: i32,
}

impl<'a> OtherGrid<'a>{
    pub fn summarize(&self, grid: &Grid) -> CheckButton{
        let select = CheckButton::new();
        let logo = Image::builder()
            .icon_name(&*self.logo)
            .use_fallback(true)
            .pixel_size(50)
            .build();
        let name = Label::new(Some(&self.name));
        let option = Label::new(Some(&self.option));
        let author = Label::new(Some(&self.author));

        grid.attach(&select, 0, self.row_num, 1, 1);
        grid.attach(&logo, 1, self.row_num, 1, 1);
        grid.attach(&name, 2, self.row_num, 1, 1);
        grid.attach(&option, 3, self.row_num, 1, 1);
        grid.attach(&author, 4, self.row_num, 1, 1);

        select
        
    }
}

pub fn clickedbutton(programm: Vec<CheckButton>, cmd: Vec<&str>) {
    for i in 0..2 {
        if programm[i].is_active(){
            println!("{}", cmd[i])
        }
    }
}