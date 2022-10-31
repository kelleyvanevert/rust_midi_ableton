use coremidi::{Destination, EventBuffer, Protocol};
use crossterm::{cursor, style, terminal, ExecutableCommand};
use std::time::Duration;
use std::{collections::HashMap, io::stdout};
use tokio::{
    sync::broadcast::{self, Receiver, Sender},
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
    let (send_info, mut recv_info) = broadcast::channel::<(String, usize)>(16);

    spawn_pattern_thread(
        "kick".into(),
        send_info.clone(),
        tx.subscribe(),
        kick,
        0x56,
        Pattern::from("X..X..X.X.."),
    );

    spawn_pattern_thread(
        "snare".into(),
        send_info.clone(),
        tx.subscribe(),
        snare_1,
        0x40,
        Pattern::from("...X....X.."),
    );

    spawn_pattern_thread(
        "closed_hat".into(),
        send_info.clone(),
        tx.subscribe(),
        closed_hat,
        0x50,
        euclid(5, 8).fill(22),
    );

    spawn_pattern_thread(
        "side_stick".into(),
        send_info.clone(),
        tx.subscribe(),
        side_stick,
        0x20,
        euclid(6, 10).rotate(1).fill(22),
    );

    spawn_pattern_thread(
        "open_hat_1".into(),
        send_info.clone(),
        tx.subscribe(),
        open_hat_1,
        0x40,
        euclid(7, 12).rotate(1).fill(22),
    );

    spawn_pattern_thread(
        "ride_cup".into(),
        send_info.clone(),
        tx.subscribe(),
        ride_cup,
        0x30,
        euclid(6, 10).rotate(1).fill(22),
    );

    tokio::spawn(async move {
        let mut timeline: HashMap<String, Vec<usize>> = HashMap::new();

        loop {
            let (name, i) = recv_info.recv().await.unwrap();
            let offset = 10_usize;
            let min = i.saturating_sub(32);

            timeline
                .entry(name)
                .and_modify(|e| {
                    e.push(i);
                })
                .or_insert(vec![i]);

            let mut stdout = stdout();

            stdout
                .execute(terminal::Clear(terminal::ClearType::All))
                .unwrap();

            for (i, (name, hits)) in timeline.iter().enumerate() {
                stdout
                    .execute(cursor::MoveTo(0, i as u16))
                    .unwrap()
                    .execute(style::Print(name))
                    .unwrap();

                for &hit in hits {
                    if hit >= min {
                        stdout
                            .execute(cursor::MoveTo((offset + hit - min) as u16 * 2, i as u16))
                            .unwrap()
                            .execute(style::Print("X"))
                            .unwrap();
                    }
                }
            }
        }
    });

    for i in 1.. {
        sleep(Duration::from_millis(200)).await;
        tx.send(i).unwrap();
    }
}

fn spawn_pattern_thread(
    name: String,
    send_info: Sender<(String, usize)>,
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
            if b == 1 {
                send_info.send((name.clone(), i)).unwrap();
                output_port.send(&destination, &note_off).unwrap();
                output_port.send(&destination, &note_on).unwrap();
            }
        }

        true
    })
}
