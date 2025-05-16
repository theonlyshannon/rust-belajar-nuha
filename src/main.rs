// Import modul 1602025
mod modul_1602025;

fn main() {
    println!("Hello, world!");

    let x = 5; // immutable nilainya tidak bisa diubah setelah didefinisikan
    let mut y = 10; // mutable Mutable Variable (Bisa Diubah)
    println!("x: {}, y: {}", x, y);

    // tipe data primive
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let arr = [1, 2, 3, 4, 5];
    println!("tup: {:?}", tup);
    println!("arr: {:?}", arr);
    
    println!("\n----- Demo Kondisional -----");
    modul_1602025::demo_conditional();
    
    println!("\n----- Demo Loops -----");
    modul_1602025::demo_loops();
    
    println!("\n----- Demo Greet -----");
    modul_1602025::greet("Firda");
    
    println!("\n----- Demo Shadowing -----");
    modul_1602025::demo_shadowing();
    
    println!("\n----- Demo Ownership -----");
    modul_1602025::demo_ownership();
    
    println!("\n----- Demo Clone -----");
    modul_1602025::demo_clone();
    
    println!("\n----- Demo Borrowing -----");
    modul_1602025::demo_borrowing();
    
    println!("\n----- Demo Mutable Borrowing -----");
    modul_1602025::demo_mutable_borrowing();
    
    println!("\n----- Demo String Slices -----");
    modul_1602025::demo_string_slices();
    
    println!("\n----- Demo Lifetime -----");
    modul_1602025::demo_lifetime();
}
