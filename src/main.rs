
mod connect_four;

use connect_four::intro::start_sequence;
use connect_four::gameloop::start_gameloop;

fn main() {
    start_sequence();
    start_gameloop();
}
