#![allow(dead_code)]
mod modules;
mod super_self;
mod uses;

pub fn run() {
    //
    modules::run();
    uses::run();
    super_self::run();
}
