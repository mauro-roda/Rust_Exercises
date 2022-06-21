/*
    Exercise from:
    https://adventofcode.com/2021/day/1
*/

fn main() {
    let depths: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    for i in 0..depths.len() { // Loop through array
        if i != 0 { // Discard first element so it doesn't panic, since there is no previous value to compare to
            if depths[i] > depths[i-1] {
                println!("{} (increased)", depths[i]);
            } else if depths[i] < depths[i-1] {
                println!("{} (decreased)", depths[i]);
            }
        } else {
            println!("{} (N/A - no previous measurement)", depths[i]);
        }
    }
}
