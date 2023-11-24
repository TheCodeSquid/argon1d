use std::{
    fs,
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
};

use anyhow::{anyhow, Result};

const SOCKET_PATH: &str = "/run/argon1d/argon1d.sock";

#[derive(Clone, Copy)]
pub enum Message {
    Stop,
    Fan(u8),
}

impl Message {
    pub fn send(self) -> Result<()> {
        let mut stream = UnixStream::connect(SOCKET_PATH)?;
        match self {
            Message::Stop => stream.write_all(&[0])?,
            Message::Fan(speed) => stream.write_all(&[1, speed])?,
        }
        Ok(())
    }
}

pub struct Listener(UnixListener);

impl Listener {
    pub fn new() -> Result<Self> {
        let listener = UnixListener::bind(SOCKET_PATH)?;
        Ok(Self(listener))
    }

    pub fn accept(&mut self) -> Result<Message> {
        let (mut stream, _) = self.0.accept()?;

        let mut msg = [0];
        stream.read_exact(&mut msg)?;
        match msg[0] {
            0 => Ok(Message::Stop),
            1 => {
                let mut speed = [0];
                stream.read_exact(&mut speed)?;

                Ok(Message::Fan(speed[0]))
            }
            _ => Err(anyhow!("Invalid message byte")),
        }
    }

    pub fn close(self) -> Result<()> {
        fs::remove_file(SOCKET_PATH)?;
        Ok(())
    }
}
