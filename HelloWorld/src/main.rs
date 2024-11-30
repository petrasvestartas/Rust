#[derive(Debug)]
struct MigrosBag {
    cakes: Vec<String>,
}

fn main() {

    // Lists
    let types = vec!["round", "triangle", "quad", "cylindrical"];
    let values = vec!["2", "3", "4", "5", "6", "7", "8", "9", "10", "A", "B", "C", "D"];

    // Loops
    let mut cakes = vec![];
    for suit in &types {
        for value in &values {
            let cake = format!("{} of {}", value, suit);
            cakes.push(cake);
        }
    }

    // Struct instance
    let migros_bag: MigrosBag = MigrosBag { cakes };

    // Print formatting
    println!("Here is your MigrosBag you paid 0.4 CHF: {:#?}", migros_bag);
}