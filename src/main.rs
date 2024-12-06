/**
 * Main entry point for blukey
 * @author shashankx86
 * @license MIT
 */

mod config;
mod cli;
mod keyboard;
mod daemon;

use clap::Parser;
use std::process::{self, Command};
use nix::unistd::geteuid;

use crate::cli::{Cli, Commands, KeyCommands};
use crate::config::{get_config_path, load_config};
use crate::keyboard::{monitor_keyboard, register_new_key};

fn main() {
    let config = load_config();
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Start) => {
            if !geteuid().is_root() {
                eprintln!("Root privileges required");
                process::exit(1);
            }

            if config.demon {
                println!("Starting daemon...");
                if let Err(e) = daemon::start_daemon() {
                    eprintln!("Error starting daemon: {}", e);
                    process::exit(1);
                }
                monitor_keyboard(config);
            } else {
                println!("Starting in normal mode...");
                monitor_keyboard(config);
            }
        }
        Some(Commands::Stop) => {
            if let Ok(pid) = std::fs::read_to_string("/tmp/blukey.pid") {
                Command::new("kill")
                    .arg(pid.trim())
                    .status()
                    .unwrap();
                println!("Daemon stopped");
            } else {
                println!("No daemon running");
            }
        }
        Some(Commands::Key(key_cmd)) => {
            match key_cmd {
                KeyCommands::New => register_new_key(),
                KeyCommands::List => {
                    if config.keys.is_empty() {
                        println!("No key combinations registered");
                        return;
                    }
                    
                    println!("\nRegistered Key Combinations:");
                    println!("{:-<80}", "");
                    println!("{:<40} | {:<37}", "Key Combination", "Command");
                    println!("{:-<80}", "");
                    
                    for (combo, cmd) in config.keys {
                        println!("{:<40} | {:<37}", combo, cmd);
                    }
                    println!("{:-<80}", "");
                }
            }
        }
        Some(Commands::ConfigPath) => println!("{}", get_config_path().display()),
        None => {
            println!("blukey v0.0.1-rc0 - Keyboard shortcut daemon\n");
            println!("Usage:");
            println!("  blukey <COMMAND>\n");
            println!("Commands:");
            println!("  start         Start the keyboard monitor");
            println!("  stop          Stop the daemon");
            println!("  key           Manage key combinations");
            println!("  config-path   Show config file location\n");
            println!("Config path: {}", get_config_path().display());
        }
    }
}