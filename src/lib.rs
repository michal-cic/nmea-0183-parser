#![no_std]

use core::str;
use heapless::Vec;

fn is_rmc_valid(rmc_sentence: &str) -> bool {
    let slices: Vec<&str, 32> = rmc_sentence.split(',').collect();

    if slices.len() >= 7 && slices[2] == "V" {
        false
    } else {
        true
    }
}

fn is_gga_valid(gga_sentence: &str) -> bool {
    let slices: Vec<&str, 32> = gga_sentence.split(',').collect();

    if slices.len() >= 7 && slices[6] == "0" {
        false
    } else {
        true
    }
}

fn is_gll_valid(gll_sentence: &str) -> bool {
    let slices: Vec<&str, 32> = gll_sentence.split(',').collect();

    if slices.len() >= 7 && slices[6] == "V" {
        false
    } else {
        true
    }
}

pub fn is_valid(message: &str) -> bool {
    let mut valid = false;
    let mut rmc: Option<&str> = None;
    let mut gga: Option<&str> = None;
    let mut gll: Option<&str> = None;
    let lines = message.lines();

    for line in lines {
        if line.contains("RMC") {
            rmc = Some(line.trim());
        }
        if line.contains("GGA") {
            gga = Some(line.trim());
        }
        if line.contains("GLL") {
            gll = Some(line.trim());
        }
    }

    // Check RMC validity
    if let Some(rmc_val) = rmc {
        valid = is_rmc_valid(rmc_val);
    }

    // Check GGA validity
    if let Some(gga_val) = gga {
        valid = is_gga_valid(gga_val);
    }

    // Check GLL validity
    if let Some(gll_val) = gll {
        valid = is_gll_valid(gll_val);
    }

    valid
}
