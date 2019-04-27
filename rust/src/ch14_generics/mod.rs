#![allow(dead_code)]
mod bounds;
mod gen_funcs_and_impl;
mod generics;
mod traits;
mod where_clauses;

pub fn run() {
    //
    generics::run();
    gen_funcs_and_impl::run();
    traits::run();
    bounds::run();
    where_clauses::run();
}
