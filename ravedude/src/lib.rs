mod avrdude;
mod board;
mod ui;
mod console;

pub use avrdude::Avrdude;
pub use avrdude::config::BoardAvrdudeOptions;
pub use board::get_board_from_name;
pub use console::open as attach_console;