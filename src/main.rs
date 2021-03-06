/* Variables Exercise */
// const STARTING_MISSILES: i32 = 8;
// const READY_AMOUNT: i32 = 2;
//
// fn main() {
//     let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
//     println!("Firing {} of my {} missiles...", ready, missiles);
//     missiles = missiles - ready;
//     println!("{} missiles left", missiles);
// }

/* Functions Exercise */
// fn main() {
//     let width = 4;
//     let height = 7;
//     let depth = 10;
//     let area = area_of(width, height);
//
//     println!("Area is {}", area);
//     println!("Volume is {}", volume_of(width, height, depth));
// }
//
// fn area_of(x: i32, y: i32) -> i32 {
//     x * y
// }
//
// fn volume_of(x: i32, y: i32, z: i32) -> i32 {
//     x * y * z
// }

/* Simple Types Exercise */
// use hello_rust::{
//     print_difference,
//     print_array,
//     ding,
//     on_off,
//     print_distance,
// };
//
//
// fn main() {
//     let coords: (f32, f32) = (6.3, 15.0);
//     let coords_arr: [f32; 2] = [coords.0, coords.1];
//     print_difference(coords.0, coords.1);
//     print_array(coords_arr);
//
//     let series = [1, 1, 2, 3, 5, 8, 13];
//     ding(series[6]);
//
//     let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
//     on_off(mess.2[1].0);
//
//     print_distance(coords);
// }

/* Control Flow and Strings */
// Silence some warnings so they don't distract from the exercise.
// #![allow(dead_code, unused_mut, unused_variables)]
//
// fn main() {
//     let args: Vec<String> = std::env::args().skip(1).collect();
//
//     for arg in args {
//         if arg == "sum" {
//             sum()
//         } else if arg == "double" {
//                 double()
//         } else {
//             count(arg)
//         };
//     }
// }
//
// fn sum() {
//     let mut sum = 0;
//     for num in 7..=23 {
//         sum += num
//     }
//
//     println!("The sum is {}", sum);
// }
//
// fn double() {
//     let mut count = 0;
//     let mut x = 1;
//     while x < 500 {
//         x *= 2;
//         count += 1;
//     }
//
//     println!("You can double x {} times until x is larger than 500", count);
// }
//
// fn count(arg: String) {
//     let mut count = 0;
//     loop {
//         println!("{}", arg);
//         count += 1;
//         if count == 8 {
//             break;
//         }
//     };
//
//     println!();
// }

/* Ownership & References Exercise */
// Silence some warnings so they don't distract from the exercise.
// #![allow(unused_mut)]
//
// fn main() {
//     let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
//         println!("Please supply an argument to this program.");
//         std::process::exit(-1);
//     });
//
//     inspect(&arg);
//
//     change(&mut arg);
//     println!("I have many {}", arg);
//
//     if eat(arg) {
//        println!("Might be bananas");
//     } else {
//        println!("Not bananas");
//     }
//
//     let arg1: i32 = std::env::args().nth(2).unwrap().parse().unwrap();
//     let arg2: i32 = std::env::args().nth(3).unwrap().parse().unwrap();
//     println!("1 + 2 = {}, even via references", add(&arg1, &arg2));
// }
//
// fn inspect(s: &String) {
//     if s.ends_with("s") {
//         println!("{} is plural.", s);
//     } else {
//         println!("{} is singular.", s);
//     };
// }
//
// fn change(s: &mut String) {
//     if !s.ends_with("s") {
//         s.push_str("s")
//     }
// }
//
// fn eat(s: String) -> bool {
//     s.starts_with("b") && s.contains("a")
// }
//
// fn add(arg1: &i32, arg2: &i32) -> i32 {
//     *arg1 + *arg2
// }

/* Structs & Traits Exercise */
// fn main() {
//     // Once you finish #1 above, this part should work.
//     let mut carrot = Carrot { percent_left: 100.0 };
//     carrot.bite();
//     println!("I take a bite: {:?}", carrot);
//
//     let mut grapes = Grapes { amount_left: 100 };
//     grapes.bite();
//     println!("Eat a grape: {:?}", grapes);

//     bunny_nibbles(&mut carrot);
//     println!("Bunny nibbles for awhile: {:?}", carrot);
//
//     bunny_nibbles(&mut grapes);
//     println!("Monkey nibbles for awhile: {:?}", grapes);
// }
//
// #[derive(Debug)] // This enables using the debugging format string "{:?}"
// struct Carrot {
//     percent_left: f32,
// }
//
// trait Bite {
//     fn bite(self: &mut Self);
// }
//
// impl Bite for Carrot {
//     fn bite(self: &mut Self) {
//         // Eat 20% of the remaining carrot. It may take awhile to eat it all...
//         self.percent_left *= 0.8;
//     }
// }
//
// #[derive(Debug)]
// struct Grapes {
//     amount_left: i32,
// }
//
// impl Bite for Grapes {
//     fn bite(self: &mut Self) {
//         self.amount_left -= 5;
//     }
// }
//
// fn bunny_nibbles<T: Bite>(item: &mut T) {
//     for _ in 0..3 {
//         item.bite();
//     }
// }

/* Collections & Enums Exercise */
// Silence some warnings that could distract from the exercise
// #![allow(unused_variables, unused_mut, dead_code)]
//
// enum Shot {
//     Bullseye,
//     Hit(f64),
//     Miss,
// }
//
// impl Shot {
//     fn points(self) -> i32 {
//         match self {
//             Shot::Bullseye => 5,
//             Shot::Hit(x) if x < 3.0 => 2,
//             Shot::Hit(x) => 1,
//             Shot::Miss => 0,
//         }
//     }
// }
//
// fn main() {
//     let arrow_coords: Vec<Coord> = get_arrow_coords(5);
//     let mut shots: Vec<Shot> = Vec::new();
//
//     for coord in arrow_coords {
//         match coord.distance_from_center() {
//             x if x < 1.0 => shots.push(Shot::Bullseye),
//             x if x < 5.0 => shots.push(Shot::Hit(x)),
//             _ => shots.push(Shot::Miss),
//         };
//     }
//
//
//     let mut total = 0;
//     for shot in shots {
//         total += shot.points()
//     }
//
//     println!("Final point total is: {}", total);
// }
//
// #[derive(Debug)]
// struct Coord {
//     x: f64,
//     y: f64,
// }
//
// impl Coord {
//     fn distance_from_center(&self) -> f64 {
//         (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
//     }
//     fn print_description(&self) {
//         println!(
//             "coord is {:.1} away, at ({:.1}, {:.1})",
//             self.distance_from_center(),
//             self.x,
//             self.y);
//     }
//
// }
//
// fn get_arrow_coords(num: u32) -> Vec<Coord> {
//     let mut coords: Vec<Coord> = Vec::new();
//     for _ in 0..num {
//         let coord = Coord {
//             x: (rand::random::<f64>() - 0.5) * 12.0,
//             y: (rand::random::<f64>() - 0.5) * 12.0,
//         };
//         coords.push(coord);
//     }
//     coords
// }

/* Closures and Threads */
#![allow(dead_code, unused_imports, unused_variables)]
use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn expensive_sum(v: Vec<i32>) -> i32 {
    pause_ms(500);
    println!("Child thread: just about finished");
    v.iter()
        .filter(|&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum()
}

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    // let my_vector = vec![2, 5, 1, 0, 4, 3];
    // let handle = thread::spawn(move || {
    //     expensive_sum(my_vector)
    // });
    //
    // for letter in vec!["a", "b", "c", "d", "e", "f"] {
    //     println!("Main thread: Letter {}", letter);
    //     pause_ms(200);
    // }
    //
    // let sum = handle.join().unwrap();
    // println!("The child thread's expensive sum is {}", sum);
    //
    // let (tx, rx) = channel::unbounded();
    // // Cloning a channel makes another variable connected to that end of the channel so that you can
    // // send it to another thread.
    // let tx2 = tx.clone();
    // let handle_a = thread::spawn(move || {
    //     pause_ms(200);
    //     tx2.send("Thread A: 1").unwrap();
    //     pause_ms(0);
    //     tx2.send("Thread A: 2").unwrap();
    // });
    // pause_ms(0); // Make sure Thread A has time to get going before we spawn Thread B
    // let handle_b = thread::spawn(move || {
    //     pause_ms(0);
    //     tx.send("Thread B: 1").unwrap();
    //     pause_ms(0);
    //     tx.send("Thread B: 2").unwrap();
    // });
    // // Using a Receiver channel as an iterator is a convenient way to get values until the channel
    // // gets closed.  A Receiver channel is automatically closed once all Sender channels have been
    // // closed.  Both our threads automatically close their Sender channels when they exit and the
    // // destructors for the channels get automatically called.
    // for msg in rx {
    //     println!("Main thread: Received {}", msg);
    // }
    // // Join the child threads for good hygiene.
    // handle_a.join().unwrap();
    // handle_b.join().unwrap();

    let (tx, rx) = channel::unbounded();
    let rx2 = rx.clone();
    let handler_1 = thread::spawn(move || {
        for msg in rx {
            pause_ms(50);
            println!("Handler 1: Received {}", msg);
        }
    });
    let handler_2 = thread::spawn(move || {
        for msg in rx2 {
            pause_ms(50);
            println!("Handler 2: Received {}", msg);
        }
    });

    let vector = vec!["a", "b", "c", "d", "e", "f"];

    for i in 0..=5 {
        if i > 3 {
            tx.send(vector[i]).unwrap()
        } else {
            println!("{}", vector[i])
        };
    }
    // Join the child threads for good hygiene.
    drop(tx);
    handler_1.join().unwrap();
    handler_2.join().unwrap();


    // Challenge: Make two child threads and give them each a receiving end to a channel.  From the
    // main thread loop through several values and print each out and then send it to the channel.
    // On the child threads print out the values you receive. Close the sending side in the main
    // thread by calling `drop(tx)` (assuming you named your sender channel variable `tx`).  Join
    // the child threads.
    println!("Main thread: Exiting.")
}
