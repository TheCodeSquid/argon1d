#[macro_use]
extern crate log;

mod config;
mod fan;
mod message;
mod thermal;

use std::{fs, path::Path, process, thread, time::Duration};

use anyhow::{anyhow, bail, Context, Result};
use clap::{Parser, Subcommand};
use config::Config;
use daemonize::Daemonize;
use rppal::i2c::I2c;
use signal_hook::{consts::SIGTERM, iterator::Signals};
use syslog::{BasicLogger, Facility, Formatter3164};

use crate::message::{Listener, Message};

const PID_FILE: &str = "/run/argon1d/argon1d.pid";
const ADDR_FAN: u16 = 0x1a;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Set fan speed
    Fan { speed: u8 },
    /// Run the monitoring service
    Service,
    /// Stops the running service
    Stop,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = config::load()?;

    match args.command {
        Command::Service => service(config),
        Command::Fan { speed } => Message::Fan(speed).send(),
        Command::Stop => Message::Stop.send(),
    }
}

fn service(config: Config) -> Result<()> {
    let path = Path::new(PID_FILE);
    if path.exists() {
        bail!("Daemon already started");
    }
    let parent = path.parent().unwrap();
    if !parent.exists() {
        fs::create_dir(parent)?;
    }

    Daemonize::new().pid_file(PID_FILE).start()?;

    let formatter = Formatter3164 {
        facility: Facility::LOG_DAEMON,
        hostname: None,
        process: "argon1d".to_string(),
        pid: process::id(),
    };
    let logger =
        syslog::unix(formatter).map_err(|err| anyhow!("Failed to connect to syslog: {:?}", err))?;

    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|_| log::set_max_level(log::LevelFilter::Info))?;

    let i2c = i2c()?;
    fan::set_speed(&i2c, 0)?;

    let mut listener = Listener::new()?;

    thread::spawn(move || -> Result<()> {
        let mut signals = Signals::new([SIGTERM])?;
        if signals.forever().next().is_some() {
            info!("Received stop signal");
            Message::Stop.send()?;
        }
        Ok(())
    });

    thread::spawn(move || -> Result<()> {
        let mut current_speed = 0;

        loop {
            let current_temp = thermal::temp()?;
            let speed = config
                .fan
                .iter()
                .find(|(temp, _)| current_temp >= **temp)
                .map(|(_, speed)| *speed)
                .unwrap_or_default();

            if speed < current_speed {
                thread::sleep(Duration::from_secs(config.slow_delay));
            }
            if speed != current_speed {
                current_speed = speed;
                Message::Fan(speed).send()?;
            } else {
                debug!("Fan speed unchanged");
            }

            thread::sleep(Duration::from_secs(config.poll_time));
        }
    });

    loop {
        let msg = match listener.accept() {
            Ok(msg) => msg,
            Err(_err) => {
                todo!() // log error
            }
        };

        match msg {
            Message::Fan(speed) => fan::set_speed(&i2c, speed)?,
            Message::Stop => break,
        }
    }

    fs::remove_file(PID_FILE)?;
    listener.close()?;

    info!("Stopped");
    Ok(())
}

fn i2c() -> Result<I2c> {
    let mut bus = I2c::new().with_context(|| "I2C disabled or permission denied")?;
    bus.set_slave_address(ADDR_FAN)
        .with_context(|| "Failed to set I2C address")?;
    Ok(bus)
}
