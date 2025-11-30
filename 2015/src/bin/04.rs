use std::io;
use md5;
use std::thread;
use std::sync::mpsc;
use std::time;

fn run(input: &str, prefix: &str) {
    let key = input.trim();

    for i in 1.. {
        let to_hash = format!("{}{}", key, i);
        let digest = md5::compute(&to_hash);
        let digest_hex = format!("{:x}", digest);
        if digest_hex.starts_with(prefix) {
            println!("{}", i);
            return;
        }
    }
}

fn run_concurrent(input: &str, prefix: &str) {
    let (res_tx, res_rx) = mpsc::channel();

    let mut task_channels = Vec::new();
    let mut threads = Vec::new();
    for _i in 0..4 {
        let (task_tx, task_rx) = mpsc::sync_channel(2);
        task_channels.push(task_tx);
        let key = input.trim().to_owned();
        let prefix = prefix.to_owned();
        let res_tx = res_tx.clone();
        let thread = thread::spawn(move || {
            for t in task_rx {
                let to_hash = format!("{}{}", key, t);
                let digest = md5::compute(&to_hash);
                let digest_hex = format!("{:x}", digest);
                if digest_hex.starts_with(&prefix) {
                    let _ = res_tx.send(t);
                    break;
                }
            }
        });
        threads.push(thread);
    }

    drop(res_tx);
    let mut results = Vec::new();

    let mut cycle_tx = task_channels.iter().cycle();

    for i in 1.. {
        if let Ok(res) = res_rx.try_recv() {
            results.push(res);
            break;
        }

        for tx in cycle_tx.by_ref() {
            if let Ok(_) = tx.try_send(i) {
                break
            }
        }
    }

    drop(task_channels);
    for t in threads {
        t.join().unwrap();
    }

    for r in res_rx {
        results.push(r);
    }

    results.sort();

    println!("{}", results.first().unwrap());
}

fn main() -> io::Result<()> {
    let input = aoc2015::read_stdin_to_string()?;

    run(&input, "00000");

    let now = time::Instant::now();
    run(&input, "000000");
    println!("{} sec", now.elapsed().as_secs());

    let now = time::Instant::now();
    run_concurrent(&input, "000000");
    println!("{} sec", now.elapsed().as_secs());

    Ok(())
}