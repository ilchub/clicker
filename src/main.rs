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
    let prereturn: u8;
    let postreturn: u8;
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    println!("Please define timeout before start: ");
    timevalue = read!();
    timeout = time::Duration::from_secs(timevalue);
    println!("Please define returns before keys: ");
    prereturn = read!();
    println!("Please define returns after keys: ");
    postreturn = read!();
    println!("Please define keys to simulate: ");
    let keys: String = read!("{}\n");
    thread::sleep(timeout);
    loop {
        if prereturn != 0 {
             while i < prereturn {
                enigo.key_down(Key::Return);
                enigo.key_up(Key::Return);
                i+=1;
            }
        }
        i = 0;
        enigo.key_sequence(&keys);
            while j < postreturn {
                enigo.key_down(Key::Return);
                enigo.key_up(Key::Return);
                j+=1;
        }
        j = 0;
    }
}
