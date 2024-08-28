mod configs;
use configs::CONSOLE_WIDTH;
use utils::wait_for_enter;

mod organize;
mod utils;
use organize::organize;

fn main() {
    println!(
        "{:=<cw$}\n{:^cw$}\n{:=<cw$}",
        "",
        "FOLDER CLEANER",
        "",
        cw = CONSOLE_WIDTH
    );
    organize();
    wait_for_enter();
}
