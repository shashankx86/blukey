/**
 * CLI command definitions for blukey
 * @author shashankx86
 * @license MIT
 */

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author = "shashankx86", version = "0.0.2-rc0", about, long_about = None)]
#[command(name = "blukey")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
#[command(rename_all = "lowercase")]
pub enum Commands {
    Start,
    Stop,
    #[command(subcommand)]
    #[command(about = "Manage key combinations")]
    Key(KeyCommands),
    ConfigPath,
}

#[derive(Subcommand)]
#[command(rename_all = "lowercase")]
pub enum KeyCommands {
    #[command(about = "Register a new key combination")]
    New,
    #[command(about = "List all registered key combinations")]
    List,
}