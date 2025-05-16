// Import modul 17052025
mod modul_17052025;

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
    
    println!("\n----- Demo Struct User -----");
    modul_17052025::demo_struct_user();
    
    println!("\n----- Demo Struct Rectangle -----");
    modul_17052025::demo_struct_rectangle();
    
    println!("\n----- Demo Enum Direction -----");
    modul_17052025::demo_enum_direction();
    
    println!("\n----- Demo Enum Message -----");
    modul_17052025::demo_enum_message();
    
    println!("\n----- Demo Option -----");
    modul_17052025::demo_option();
    
    println!("\n----- Demo Result -----");
    modul_17052025::demo_result();
    
    // Contoh mengakses struct dan fungsi langsung
    println!("\n----- Demo Membuat Rectangle Baru -----");
    let kotak = modul_17052025::Rectangle::new(20, 15);
    println!("Luas kotak: {}", kotak.area());
    
    // Contoh membuat User sendiri
    println!("\n----- Demo Membuat User Baru -----");
    let user_baru = modul_17052025::User {
        username: String::from("budi"),
        email: String::from("budi@example.com"),
        active: true,
    };
    println!("User baru: {}, {}", user_baru.username, user_baru.email);
}
