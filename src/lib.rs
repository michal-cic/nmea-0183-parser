// #![no_std]

use core::str;
use heapless::Vec;

#[derive(Debug)]
pub struct Parser {}

impl Parser {
    pub fn parse(self, sentence: &str) -> Vec<&str, 20_usize> {
        let mut s: Vec<&str, 20> = Vec::new();
        // $GPRMC,015606.000,A,3150.7584,N,11712.0491,E,0.00,231.36,280715,,,A*67<CR><LF>
        if sentence.contains("RMC") {
            s = sentence.split(',').collect();
            let rmc = RMC {
                time: s[1],
                is_valid: s[2] == "A",
                lat: s[3],
                lon: s[4],
            };
        }
        s
    }
}

pub fn init() -> Parser {
    Parser {}
}

struct RMC<'a> {
    pub time: &'a str,
    pub is_valid: bool,
    pub lat: &'a str,
    pub lon: &'a str,
}
