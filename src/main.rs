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
#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        if arg == "sum" {
            sum()
        } else if arg == "double" {
                double()
        } else {
            count(arg)
        };
    }
}

fn sum() {
    let mut sum = 0;
    for num in 7..=23 {
        sum += num
    }

    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    while x < 500 {
        x *= 2;
        count += 1;
    }

    println!("You can double x {} times until x is larger than 500", count);
}

fn count(arg: String) {
    let mut count = 0;
    loop {
        println!("{}", arg);
        count += 1;
        if count == 8 {
            break;
        }
    };

    println!();
}
