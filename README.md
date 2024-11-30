# Rust

![rust_logo](https://github.com/user-attachments/assets/909d3d19-10cf-481a-8f97-f8a39d919085)



## Installation 

### Step 1 - Install Rust via Terminal
Install Rust via [RustUp](rust-lang.org/tools/install) Package Manager.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


### Step 2 - Verify Installation

```bash
cargo --version
```

## Commands

### Create a new project
```bash
cargo new <projectname>
```

### Run a project
```bash
cargo run
```

### Run a project without Debug Info
```bash
cargo run -q
```


### Step 3 - VSCode

Extensions: [rust analyzer](https://code.visualstudio.com/docs/languages/rust)


### Step 4 - Code

#### Struct, List, Loop and Print

```rust
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
```