use std::process::Command;
use super::config;
use systemstat::{Platform, System, Network, BTreeMap};

// Updating X rootserver's window name.
pub fn setxroot(_status_text: String) {
    let _output = Command::new("xsetroot")
        .arg("-name")
        .arg(_status_text)
        .output()
        .expect("Failed to set X root window name!");
    // println!("setxroot status: {:?}", output.status);
}

// Running keyboard layout getter script.
pub fn get_keyboard_layout(
    ) -> String {
    let _output = Command::new("setxkbmap")
        .arg("-query")
        .output()
        .expect("??");
    String::from_utf8(
        _output.stdout[_output.stdout.len()-3..].to_vec()
    ).unwrap().replace("\n", "").to_uppercase()
}

// Clock stuff.
// Works with only exodwm or text-color patched dwm.
pub fn number_to_binary_str(num: u8) -> String {
    let mut binary_str: String = "".to_string();
    for bit in 0..8 {
        binary_str = match num >> bit & 0x01 {
            1 => format!("{}^c{}^●^d^", binary_str, config::ACTIVE_COLOR),
            _ => format!("{}●", binary_str),
        }
    }
    binary_str
}

// Calculating network statistics.
pub fn calculate_network_rxtx<'a>(
    sys:                 &System
    , netw:              &BTreeMap<String, Network>
    , rx_bytes_previous: &'a mut u32
    , tx_bytes_previous: &'a mut u32
    , rx_bytes_counter:  &'a mut u32
    , tx_bytes_counter:  &'a mut u32
    , rx_bytes:          &'a mut u32
    , tx_bytes:          &'a mut u32
    , rx_bytes_diff:     &'a mut i64
    , tx_bytes_diff:     &'a mut i64
    , cycle_counter:     &u8
    )
{
    let mut rx_bytes_summa = 0u32;
    let mut tx_bytes_summa = 0u32;

    for network_if in netw.values() {
        match sys.network_stats(&network_if.name) {
            Ok(netstat) => {
                rx_bytes_summa += netstat.rx_bytes as u32;
                tx_bytes_summa += netstat.tx_bytes as u32;
            }
            Err(e) => println!("{}", e),
        }
    }
    *rx_bytes_diff = rx_bytes_summa as i64 - *rx_bytes_previous as i64;
    if *rx_bytes_diff < 0 {
        *rx_bytes_diff = 0;
    }
    *tx_bytes_diff = tx_bytes_summa as i64 - *tx_bytes_previous as i64;
    if *tx_bytes_diff < 0 {
        *tx_bytes_diff = 0;
    }
    *rx_bytes_counter += *rx_bytes_diff as u32;
    *tx_bytes_counter += *tx_bytes_diff as u32;
    *rx_bytes_previous = rx_bytes_summa;
    *tx_bytes_previous = tx_bytes_summa;
    if (*cycle_counter as u16) % (1000 / (config::CYCLE_LENGTH as u16)) == 0 {
        *rx_bytes = *rx_bytes_counter;
        *tx_bytes = *tx_bytes_counter;
        *rx_bytes_counter = 0;
        *tx_bytes_counter = 0;
    }
}

pub fn get_battery_pwr(sys: &System) -> u8 {
    match sys.battery_life() {
        Ok(battery) => {
            (battery.remaining_capacity * 100.0) as u8
        }
        Err(_e) => {0u8}
    }
}

pub fn get_battery_ac(sys: &System) -> bool {
    match sys.on_ac_power() {
        Ok(is_ac_plugged) => {
            is_ac_plugged
        }
        Err(_e) => {false}
    }
}

