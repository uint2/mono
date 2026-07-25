mod c;
mod logger;

use log::LevelFilter;

fn main() {
    logger::init(LevelFilter::Debug);

    unsafe { c::XOpenDisplay(std::ptr::null()) };

    log::info!("Helloworld");
}
