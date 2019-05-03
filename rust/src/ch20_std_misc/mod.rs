mod channels;
mod child_processes;
mod fileIO;
mod paths;
mod threads;

pub fn run() {
    threads::run();
    channels::run();
    paths::run();
    fileIO::run();
    child_processes::run();
}
