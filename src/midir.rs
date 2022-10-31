use crate::euclid::*;
use midir::{MidiOutput, MidiOutputPort};
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

pub fn play() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let beats = euclid(5, 8);

    const NOTE_ON_MSG: u8 = 0x90;
    const NOTE_OFF_MSG: u8 = 0x80;
    const VELOCITY: u8 = 0x64;
    const DUR: u64 = 200;

    let midi_out = MidiOutput::new("My Test Output")?;

    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => return Err("no output port found".into()),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            &out_ports[0]
        }
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            out_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid output port selected")?
        }
    };

    let wait = |dur: u64| {
        sleep(Duration::from_millis(dur));
    };

    println!("\nOpening connection");
    let mut conn_out = midi_out.connect(out_port, "midir-test")?;
    println!("Connection open. Listen!");

    for b in beats.clone().into_iter().cycle() {
        for note in 10..99 {
            let _ = conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);
        }
        if b == 1 {
            for note in [58, 62, 65] {
                let _ = conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]);
            }
        }
        wait(DUR);
    }
    // play_note(66, 4);
    // play_note(65, 3);
    // play_note(63, 1);
    // play_note(61, 6);
    // play_note(59, 2);
    // play_note(58, 4);
    // play_note(56, 4);
    // play_note(54, 4);

    wait(DUR);
    println!("\nClosing connection");
    // This is optional, the connection would automatically be closed as soon as it goes out of scope
    conn_out.close();
    println!("Connection closed");
    Ok(())
}
