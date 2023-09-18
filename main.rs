fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("Inner: {x}");
    }

    println!("Basic: {x}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{heart_eyed_cat}");
}
