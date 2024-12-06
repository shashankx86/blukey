/**
 * Daemon mode handling for blukey
 * @author shashankx86
 * @license MIT
 */

use std::fs::File;
use daemonize::Daemonize;

pub fn start_daemon() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = File::create("/tmp/blukey.out")?;
    let stderr = File::create("/tmp/blukey.err")?;

    let daemonize = Daemonize::new()
        .pid_file("/tmp/blukey.pid")
        .chown_pid_file(true)
        .working_directory("/tmp")
        .stdout(stdout)
        .stderr(stderr);

    daemonize.start()?;
    Ok(())
}