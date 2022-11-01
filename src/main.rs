
mod intro;
mod gameloop;
mod utility;

use intro::start_sequence;
use gameloop::start_gameloop;
use utility::clear_all;

fn main() {
    start_sequence(clear_all);
    start_gameloop(clear_all);
}
