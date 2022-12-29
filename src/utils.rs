use std::{
    env,
    process,
};

use log::error;
use poise::serenity_prelude::Color;

pub fn string_from_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| {
        error!("Expected the key '{name}' to be set in the environment.");
        process::exit(1); // -> exiting with error and cleaning code out to be safer 
    })
}

pub fn color_from_env(name: &str) -> Color {
    let raw = &string_from_env(name);

    Color::from(u32::from_str_radix(raw, 16).unwrap_or_else(|_| {
        error!(
            "Expected the value for the color key '{name}' to be a valid hex code (Got '{raw}'.)."
        );
        process::exit(1); // -> Same thing on top
    }))
}

pub fn cut_excess(mut string: String, n: usize) -> String {
    if string.len() > n {
        string.truncate(n);
        format!("{string}…")
    }
    else {
        string
    }
}
