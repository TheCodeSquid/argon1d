# argon1d

An alternative daemon for the Argon ONE Raspberry Pi case.

## Why does this exist?

Argon40's provided scripts don't work on Alpine Linux (they use systemd), so I decided to make my own.
This version is implemented in Rust rather than Python, so it also benefits from not requiring several runtime dependencies.

## Installation

### Alpine Linux
- Enable I2C
  - Add `dtparam=i2c_arm=on` to `/boot/usercfg.txt`
  - Add `i2c-dev` to `/etc/modules`
- Add the `argon1d` OpenRC service
  - Install [`openrc/argon1d`](openrc/argon1d) to `/etc/init.d/`
  - Run `rc-update add argon1d` as a superuser
- Install compiled binary to `/usr/bin/argon1d`

## TODO:

Currently lacks power button functionality.

*Coming soon*
