#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_ui;
mod style;

use crate::app_ui::TheApplication;

use iced::{Application, Settings};

fn main() -> iced::Result {
    TheApplication::run(Settings::default())
}