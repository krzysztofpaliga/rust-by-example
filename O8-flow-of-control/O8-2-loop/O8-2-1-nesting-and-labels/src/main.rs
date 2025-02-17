#![allow(unreachable_code, unused_labels)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // this would break only the inner loop
            // break;

            // this breaks the outer loop
            break 'outer;
        }

        println!("this point will never be reached");
    }

    println!("Exited the outer loop");
}
