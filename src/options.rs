/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-05-09
 */
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command()]
pub struct Options {
    /// The file to process
    pub file: PathBuf,

    /// The minimum string size
    #[arg(default_value_t = 3)]
    pub minimum: usize,
}
