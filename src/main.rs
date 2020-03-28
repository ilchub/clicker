use enigo::*;
use text_io::*;
use clap::{App};
use std::{thread, time};
extern crate clap;
fn main() {
    println!("Clicker is starting...");
    let mut enigo = Enigo::new();
    let _matches = App::new("Clicker")
                          .version("1.0.0")
                          .author("Illia `drag0nhaze` Chub <ilchub5@protonmail.com>")
                          .about("Simulates layout-based user input")
                          .get_matches();
    let timevalue;
    let timeout;
    println!("Please define timeout before start: ");
    timevalue = read!();
    timeout = time::Duration::from_secs(timevalue);
    println!("Please define keys to simulate: ");
    let keys: String = read!("{}\n");
    thread::sleep(timeout);
    loop {
    enigo.key_sequence(&keys);
    }
}
