#![allow(dead_code)]
mod constants;
mod enums;
mod linked_list;
mod structures;

pub fn run() {
    structures::run();
    enums::run();
    linked_list::run();
    constants::run();
}
