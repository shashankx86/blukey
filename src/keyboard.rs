/**
 * Keyboard device handling and monitoring
 * @author shashankx86
 * @license MIT
 */

use evdev::{Device, Key};
use std::fs;
use std::io::{self, Write};
use std::process::{self, Command};
use crate::config::{Config, load_config, save_config};

pub fn find_keyboard_device() -> Option<Device> {
    for entry in fs::read_dir("/dev/input").ok()? {
        let entry = entry.ok()?;
        if entry.file_name().to_str()?.starts_with("event") {
            if let Ok(device) = Device::open(entry.path()) {
                if device.supported_keys().map_or(false, |keys| keys.contains(Key::KEY_A)) {
                    println!("Found keyboard at: {:?}", entry.path());
                    return Some(device);
                }
            }
        }
    }
    None
}

pub fn register_new_key() {
    println!("Press the key combination (press Enter when done):");
    let mut keys_pressed = Vec::new();
    let mut device = find_keyboard_device().expect("No keyboard found");
    
    loop {
        if let Ok(events) = device.fetch_events() {
            for event in events {
                if event.event_type() == evdev::EventType::KEY {
                    match event.value() {
                        1 => {
                            let key = Key::new(event.code());
                            if !keys_pressed.contains(&format!("{:?}", key)) {
                                keys_pressed.push(format!("{:?}", key));
                                println!("Added key: {:?}", key);
                            }
                        }
                        0 => {
                            if !keys_pressed.is_empty() {
                                let combo = keys_pressed.join("+");
                                println!("\nEnter command to execute for '{}': ", combo);
                                let mut command = String::new();
                                io::stdout().flush().unwrap();
                                io::stdin().read_line(&mut command).unwrap();
                                
                                let mut config = load_config();
                                config.keys.insert(combo, command.trim().to_string());
                                save_config(&config);
                                println!("Key combination registered!");
                                return;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

pub fn monitor_keyboard(config: Config) {
    let mut device = match find_keyboard_device() {
        Some(dev) => dev,
        None => {
            eprintln!("Could not find a keyboard device. Are you running as root?");
            process::exit(1);
        }
    };

    let mut current_combo = Vec::new();

    loop {
        match device.fetch_events() {
            Ok(events) => {
                for event in events {
                    if event.event_type() == evdev::EventType::KEY {
                        let key = Key::new(event.code());
                        match event.value() {
                            1 => current_combo.push(format!("{:?}", key)),
                            0 => {
                                if !current_combo.is_empty() {
                                    let combo = current_combo.join("+");
                                    if let Some(cmd) = config.keys.get(&combo) {
                                        execute_command(cmd);
                                    }
                                    current_combo.clear();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading events: {}", e);
                break;
            }
        }
    }
}

fn execute_command(command: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .unwrap();
}