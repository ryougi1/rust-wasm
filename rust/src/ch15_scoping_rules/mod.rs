#![allow(dead_code)]
mod aliasing;
mod borrowing;
mod lifetimes;
mod lifetimes_2;
mod lifetimes_3;
mod lifetimes_4;
mod ownership;
mod ref_pattern;

pub fn run() {
    //
    ownership::run();
    borrowing::run();
    aliasing::run();
    ref_pattern::run();
    lifetimes::run();
    lifetimes_2::run();
    lifetimes_3::run();
    lifetimes_4::run();
}
