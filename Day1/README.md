# Day 1: Introduction to Rust

### Setting Up GitHub Repository
Open your terminal and navigate to your desired directory where you want to create the repository.
Run the following commands to create a new repository and link it to GitHub:

```bash
mkdir rust-learning
cd rust-learning
git init
# Add a README file
echo "# Rust Learning Repository" >> README.md
git add README.md
git commit -m "Initial commit: Add README"
# Link to your GitHub repository
git remote add origin <your_github_repository_url>
git push -u origin master
```
Your GitHub repository is now set up to track your progress in learning Rust.


# Introduction to Rust
Rust is a modern systems programming language developed by Mozilla.

It's known for its safety, concurrency, and performance.
Rust emphasizes memory safety without a garbage collector, making it suitable for systems programming where low-level control is needed without sacrificing safety.

Why Rust?
Rust ensures memory safety through its ownership system, preventing common bugs like null pointer dereferences, buffer overflows, etc.

It provides fearless concurrency, allowing you to write concurrent code without worrying about data races.

Rust's performance is comparable to C and C++, making it suitable for performance-critical applications.

* Install Rust on your system using rustup. Open your terminal and run:

```bash
sudo apt update
sudo apt install build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
*Follow the instructions provided to complete the installation.*


* Verify the installation by running:

```bash
rustc --version
cargo --version
```

* Create a new Rust project using Cargo:

```bash
cargo new hello_world --bin
cd hello_world
```

Open the newly created project directory in your text editor.

You'll see a file named main.rs. Open it and replace its contents with:

```rust
fn main() {
    println!("Hello, world!");
}
```
Save the file.

* Run your program using Cargo:

```bash
cargo run
```
You should see Hello, world! printed to the console.

Congratulations! You've set up your GitHub repository and written and executed your first Rust program. Day2, we'll dive deeper into basic Rust syntax and concepts.