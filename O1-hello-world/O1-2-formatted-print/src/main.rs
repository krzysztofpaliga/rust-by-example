fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");


    println!("Base 10:              {}", 69420);
    println!("Base 2 (binary):      {:b}", 69420);
    println!("Base 8 (octal):       {:o}", 69420);
    println!("Base 16 (hexadecimal):{:x}", 69420);

    println!("{:>5}", 1);
    println!("{:0>5}", 1);
    println!("{:0<5}", 1);

    println!("{:0>width$}", 1, width=5);

    println!("Ny name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);

//    println!("This struct `{}` won't print...", Structure(3));
    
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}
