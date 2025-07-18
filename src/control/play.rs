use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum AppMessage {
    SpawnEnemy,
    MoveEnemies,
    MoveShoot,
    Shoot,
    End,
}

pub fn send_message(tx: Sender<AppMessage>, rx: Receiver<AppMessage>) {
    let (t, r) = mpsc::channel();
    for i in 0..4 {
        let t_clone = t.clone();
        match i {
            0 => {
                thread::spawn(move || {
                    loop {
                        t_clone.send(AppMessage::SpawnEnemy).expect("接收者已drop");
                        thread::sleep(Duration::from_secs(10))
                    }
                });
            }
            1 => {
                thread::spawn(move || {
                    thread::sleep(Duration::from_secs(2));
                    loop {
                        thread::sleep(Duration::from_secs(1));
                        t_clone.send(AppMessage::MoveEnemies).expect("接收者已drop")
                    }
                });
            }
            2 => {
                thread::spawn(move || {
                    thread::sleep(Duration::from_secs(3));
                    loop {
                        thread::sleep(Duration::from_millis(4500));
                        t_clone.send(AppMessage::Shoot).expect("接收者已drop");
                    }
                });
            }
            3 => {
                thread::spawn(move || {
                    thread::sleep(Duration::from_secs(4));
                    loop {
                        thread::sleep(Duration::from_millis(1500));
                        t_clone.send(AppMessage::MoveShoot).expect("接收者已drop");
                    }
                });
            }
            _ => {}
        }
    }
    while let Ok(msg) = r.recv() {
        if let Ok(_msg) = rx.try_recv() {
            drop(tx);
            break;
        }
        tx.send(msg).unwrap();
    }
}
