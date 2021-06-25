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

#![allow(dead_code, unused_variables)]
use hello_rust::{
    print_difference,
    print_array,
    ding,
    on_off,
    print_distance,
};


fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    let coords_arr: [f32; 2] = [coords.0, coords.1];
    print_difference(coords.0, coords.1);
    print_array(coords_arr);

    let series = [1, 1, 2, 3, 5, 8, 13];
    ding(series[6]);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    on_off(mess.2[1].0);

    print_distance(coords);
}

