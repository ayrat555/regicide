//! Terminal user interface: menus, board display, and play session.

mod board;
mod input;
mod menu;
mod rules;
mod session;
mod turn;

pub use menu::run;
