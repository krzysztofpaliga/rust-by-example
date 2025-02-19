
fn main() {
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{i}`. Try again.");
            optional = Some(i+1);
        }
    }
}

// loop {
//     match optional {
//         // If `optional` destructures, evaluate the block.
//         Some(i) => {
//             if i > 9 {
//                 println!("Greater than 9, quit!");
//                 optional = None;
//             } else {
//                 println!("`i` is `{:?}`. Try again.", i);
//                 optional = Some(i + 1);
//             }
//             // ^ Requires 3 indentations!
//         },
//         // Quit the loop when the destructure fails:
//         _ => { break; }
//         // ^ Why should this be required? There must be a better way!
//     }
// }