use std::{
    env,
    process::abort,
};

use log::error;
use poise::serenity_prelude::Color;

#[inline]
pub fn string_from_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| {
        error!("Expected the key '{name}' to be set in the environment.");
        abort();
    })
}

pub fn color_from_env(name: &str) -> Color {
    let raw = &string_from_env(name);

    Color::from(u32::from_str_radix(raw, 16).unwrap_or_else(|_| {
        error!(
            "Expected the value for the color key '{name}' to be a valid hex code (got '{raw}')."
        );
        abort();
    }))
}

pub fn cut_excess(string: String, n: usize) -> String {
    if string.len() > n {
        format!("{}...", &string[..n])
    }
    else {
        string
    }
}
