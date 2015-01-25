#![crate_name = "stomp"]
#![crate_type = "lib"]

#![allow(unstable)]

#[macro_use]
extern crate log;
extern crate collections;

use std::io::IoResult;
use session::Session;
use connection::Connection;

pub fn connect<'a>(ip_address: &str, port: u16) -> IoResult<Session<'a>> {
  connect_with_heartbeat(ip_address, port, 0, 0)
}

pub fn connect_with_heartbeat<'a>(ip_address: &str, port: u16, tx_heartbeat_ms: u32, rx_heartbeat_ms: u32) -> IoResult<Session<'a>> {
  let connection = try!(Connection::new(ip_address, port));
  connection.start_session(tx_heartbeat_ms, rx_heartbeat_ms)
}

pub fn connect_with_credentials<'a>(ip_address: &str, port: u16, login: &str, passcode: &str) -> IoResult<Session<'a>> {
  connect_with_credentials_and_heartbeat(ip_address, port, login, passcode, 0, 0)
}

pub fn connect_with_credentials_and_heartbeat<'a>(ip_address: &str, port: u16, login: &str, passcode: &str, tx_heartbeat_ms: u32, rx_heartbeat_ms: u32) -> IoResult<Session<'a>> {
  let connection = try!(Connection::new(ip_address, port));
  connection.start_session_with_credentials(login, passcode, tx_heartbeat_ms, rx_heartbeat_ms)
}

pub mod connection;
pub mod frame;
pub mod header;
pub mod session;
pub mod subscription;
pub mod transaction;
