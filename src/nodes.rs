use coremidi::{Destination, EventBuffer, Protocol};
use std::time::Duration;
use tokio::{
    sync::broadcast::{self, Receiver},
    task::JoinHandle,
    time::sleep,
};

use crate::euclid::{euclid, Pattern};

pub async fn play() {
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

    let (tx, _) = broadcast::channel::<usize>(16);

    spawn_pattern_thread(
        "kick".into(),
        tx.subscribe(),
        kick,
        0x64,
        Pattern::from("X..X..X.X.."),
    );

    spawn_pattern_thread(
        "snare".into(),
        tx.subscribe(),
        snare_1,
        0x40,
        Pattern::from("...X....X.."),
    );

    spawn_pattern_thread(
        "closed_hat".into(),
        tx.subscribe(),
        closed_hat,
        0x50,
        euclid(5, 8).fill(22),
    );

    spawn_pattern_thread(
        "side_stick".into(),
        tx.subscribe(),
        side_stick,
        0x20,
        euclid(6, 10).rotate(1).fill(22),
    );

    spawn_pattern_thread(
        "open_hat_1".into(),
        tx.subscribe(),
        open_hat_1,
        0x40,
        euclid(7, 12).rotate(1).fill(22),
    );

    spawn_pattern_thread(
        "ride_cup".into(),
        tx.subscribe(),
        ride_cup,
        0x40,
        euclid(6, 10).rotate(1).fill(22),
    );

    for i in 1.. {
        sleep(Duration::from_millis(200)).await;
        tx.send(i).unwrap();
    }
}

fn spawn_pattern_thread(
    name: String,
    mut rx: Receiver<usize>,
    note: u32,
    velocity: u32,
    pattern: Pattern,
) -> JoinHandle<bool> {
    tokio::spawn(async move {
        let note_on = EventBuffer::new(Protocol::Midi10)
            .with_packet(0, &[0x20000000 | 0x00900000 | note | velocity]);

        let note_off = EventBuffer::new(Protocol::Midi10)
            .with_packet(0, &[0x20000000 | 0x00800000 | note | velocity]);

        let client = coremidi::Client::new("example-client").unwrap();
        let output_port = client.output_port("example-port").unwrap();
        let destination = Destination::from_index(0).unwrap();

        for b in pattern.into_iter().cycle() {
            let i = rx.recv().await.unwrap();
            println!("[{}] recv {}", name, i);
            if b == 1 {
                output_port.send(&destination, &note_off).unwrap();
                output_port.send(&destination, &note_on).unwrap();
            }
        }

        true
    })
}
