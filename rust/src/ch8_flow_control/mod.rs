#![allow(dead_code)]

mod if_else_and_loop;
mod if_let_and_while_let;
mod matches_and_destructuring;
mod while_and_for_and_range;

pub fn run() {
    if_else_and_loop::run();
    while_and_for_and_range::run();
    matches_and_destructuring::run();
    if_let_and_while_let::run();
}
