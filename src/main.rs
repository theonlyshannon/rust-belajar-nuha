// Import modul 15052025
mod modul_15052025;

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
    modul_15052025::demo_conditional();
    
    println!("\n----- Demo Loops -----");
    modul_15052025::demo_loops();
    
    println!("\n----- Demo Greet -----");
    modul_15052025::greet("Firda");
    
    println!("\n----- Demo Shadowing -----");
    modul_15052025::demo_shadowing();
}
