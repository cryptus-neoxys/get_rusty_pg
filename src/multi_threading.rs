use std::{
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread,
    time::Duration,
    vec,
};

const NUM_TIMERS: usize = 24;

fn timer(d: usize, tx: mpsc::Sender<usize>) {
    thread::spawn(move || {
        println!("{}: sending timer . . .", d);
        thread::sleep(Duration::from_secs(d as u64));
        println!("{}: sent!!!", d);
        tx.send(d).unwrap();
    });
}

fn is_prime(n: usize) -> bool {
    (2..n).all(|i| n % i != 0)
}

fn producer(tx: mpsc::SyncSender<usize>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for i in 100_000_000.. {
            tx.send(i).unwrap();
        }
    })
}

fn worker(id: usize, shared_rx: Arc<Mutex<mpsc::Receiver<usize>>>) {
    thread::spawn(move || loop {
        {
            let mut n = 0;
            match shared_rx.lock() {
                Ok(rx) => match rx.try_recv() {
                    Ok(_n) => {
                        n = _n;
                    }
                    Err(_) => (),
                },
                Err(_) => (),
            }

            if n != 0 {
                if is_prime(n) {
                    println!("worker {} found a prime: {}", id, n);
                }
            }
        }
    });
}

fn main() {
    // // Thread spawn & join
    // let mut c = vec![];
    // for i in 0..10 {
    //     c.push(thread::spawn(move || {
    //         println!("thread number {}", i);
    //     }));
    // }
    // for j in c {
    //     j.join();
    // }

    // // `move` to take ownership of vars
    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(move || {
    //     println!("Vector {:?}", v);
    // });
    // handle.join();

    // // Channels [passing messages ~inter-thread~]
    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move || {
    //     tx.send(42).unwrap();
    // });
    // println!("got {}", rx.recv().unwrap());

    // // Message Passing cont..
    // let (tx, rx) = mpsc::channel();
    // for i in 0..NUM_TIMERS {
    //     timer(i, tx.clone());
    // }
    // for v in rx.iter().take(NUM_TIMERS) {
    //     println!("{}: received", v);
    // }

    // // Mutexes and Arc
    // let c = Arc::new(Mutex::new(0));
    // let mut hs = vec![];

    // for _ in 0..10 {
    //     let c = Arc::clone(&c);
    //     let h = thread::spawn(move || {
    //         let mut num = c.lock().unwrap();

    //         *num += 1;
    //         println!("num: {}", num);
    //     });
    //     hs.push(h);
    // }
    // for h in hs {
    //     h.join().unwrap();
    // }
    // println!("Result: {}", *c.lock().unwrap());

    // oke... finding primes
    let (tx, rx) = mpsc::sync_channel(1024);
    let shared_rx = Arc::new(Mutex::new(rx));
    for i in 1..5 {
        worker(i, shared_rx.clone());
    }

    producer(tx).join().unwrap();
}
