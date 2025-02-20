fn main() {
    let optional = Some(7);

    match optional {
        Some(i) => println!("{i}"),
        _ => {},
    }

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {i}");
    }

    if let Some(i) = letter {
        println!("Matched {i}");
    } else {
        println!("Didn't match")
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {i}")
    } else if i_like_letters {
        println!("Didn't match number")
    } else {
        println!("I dont like letters");
    }

    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }

    enum Fooo {Bar}
    let a = Fooo::Bar;
    if let Fooo::Bar = a {
        println!("a is foobar");
    }
}
