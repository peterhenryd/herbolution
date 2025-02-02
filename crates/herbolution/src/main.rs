extern crate herbolution_lib as lib;

use lib::app::App;
use lib::runtime::{RuntimeError, Runtime};
use lib::window_attributes;

fn main() -> Result<(), RuntimeError> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    Runtime::<App>::new(window_attributes(
        format!("Herbolution {}", VERSION),
        (1920, 1080)
    )).run()
}
