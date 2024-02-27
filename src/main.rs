mod group_m;
mod config;
mod io_worker;



fn main() {
    config::init();
    group_m::task_m3();
}
