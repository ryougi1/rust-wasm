#![allow(dead_code)]
mod closures;
mod closures_as_input_param;
mod closures_as_output_param;
mod input_functions;
mod methods;

pub fn run() {
    methods::run();
    closures::run();
    closures_as_input_param::run();
    input_functions::run();
    closures_as_output_param::run();
}
