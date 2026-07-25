//! Terminal user interface: menus, board display, and play session.

mod board;
mod fmt;
mod input;
mod menu;
mod rules;
mod session;
mod turn;

pub use menu::run;
