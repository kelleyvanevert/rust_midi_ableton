use coremidi::{Destination, EventBuffer, Protocol};
use std::thread::{self, sleep};
use std::time::Duration;

use crate::euclid::*;

pub fn play() {
    let dur = Duration::from_millis(200);

    let kick = 0x2400;
    let side_stick = 0x2500;
    let snare_1 = 0x2600;
    let snare_2 = 0x2700;
    let snare_3 = 0x2800;
    let tom_lo = 0x2900;
    let closed_hat = 0x2a00;
    let tom_lomid = 0x2b00;
    let open_hat_1 = 0x2c00;
    let tom_himid = 0x2d00;
    let open_hat_2 = 0x2e00;
    let tom_hi = 0x2f00;
    let crash_1 = 0x3000;
    let crash_2 = 0x3100;
    let ride = 0x3200;
    let ride_cup = 0x3300;

    let kick_pattern = Pattern::from("X..X..X.X..");
    let len = kick_pattern.len();
    let specs = vec![
        ([kick], 0x64, kick_pattern),
        ([snare_1], 0x40, Pattern::from("...X....X..")),
        ([closed_hat], 0x50, euclid(5, 8).fill(len * 2)),
        ([side_stick], 0x27, euclid(6, 10).rotate(1).fill(len * 2)),
        ([open_hat_1], 0x40, euclid(7, 12).fill(len * 2)),
        ([ride_cup], 0x40, euclid(6, 10).rotate(1).fill(len * 3)),
    ];

    for (notes, velocity, beats) in specs {
        thread::spawn(move || {
            let note_on =
                EventBuffer::new(Protocol::Midi10).with_packet(0, &chord(true, &notes, velocity));

            let note_off =
                EventBuffer::new(Protocol::Midi10).with_packet(0, &chord(false, &notes, velocity));

            let client = coremidi::Client::new("example-client").unwrap();
            let output_port = client.output_port("example-port").unwrap();
            let destination = Destination::from_index(0).unwrap();

            for b in beats.clone().into_iter().cycle() {
                output_port.send(&destination, &note_off).unwrap();
                if b == 1 {
                    output_port.send(&destination, &note_on).unwrap();
                }
                sleep(dur);
            }
        });
    }

    loop {}
}

pub fn chord<const N: usize>(on: bool, notes: &[u32; N], velocity: u32) -> [u32; N] {
    let event = if on { 0x00900000u32 } else { 0x00800000u32 };
    let msg = notes.map(|note| 0x20000000 | event | note | velocity);

    msg
}
