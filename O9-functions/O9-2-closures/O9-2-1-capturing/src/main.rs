fn main() {
    use std::mem;
    let color = String::from("green");

    let print = || println!("`color`: {color}");

    print();

    let _reborrow = &color;
    print();

    let _color_moved = color;
    // Error
    // print();

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();

    // Error
    // let _count_reborrow = &count;
    inc();

    let _count_reborrowed = &mut count;
    // Error
    // inc();

    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();

    // Error
    // consume();

    let mut movable = Box::new(3);

    let mut moves = move || {
        *movable += 1;
        println!("`movable`: {:?}", movable);
    };

    moves();

    moves();

    let haystack = vec![1, 2, 3];

    let contains = | needle | haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    println!("There're {} elements in vec", haystack.len());


    let haystack = vec![1, 2, 3];

    let contains = move | needle | haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // Error
    // println!("There're {} elements in vec", haystack.len());

    let haystack = vec![1, 2, 3];

    let mut contains = | needle | haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    println!("There're {} elements in vec", haystack.len());

    let haystack = Box::new(vec![1, 2, 3]);

    let mut contains = | needle | haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    println!("There're {} elements in vec", haystack.len());

}



