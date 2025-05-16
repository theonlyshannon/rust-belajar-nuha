pub fn demo_ownership() {
    let s1 = String::from("Rust");
    let s2 = s1; // ownership berpindah

    // println!("{}", s1); ❌ error: s1 sudah tidak memiliki ownership
    println!("{}", s2); // ✅
}

pub fn demo_clone() {
    let s1 = String::from("Rust");
    let s2 = s1.clone(); // duplikasi data

    println!("s1 = {}, s2 = {}", s1, s2); // ✅
}

fn print_len(s: &String) {
    println!("Panjang: {}", s.len());
}

pub fn demo_borrowing() {
    let s = String::from("Hello");
    print_len(&s); // s dipinjam, bukan dipindahkan
    println!("{}", s); // ✅ masih bisa dipakai
}

fn tambah_kata(s: &mut String) {
    s.push_str(" World");
}

pub fn demo_mutable_borrowing() {
    let mut s = String::from("Hello");
    tambah_kata(&mut s);
    println!("{}", s); // Hello World
}

fn cetak(s: &str) {
    println!("{}", s);
}

pub fn demo_string_slices() {
    let s1 = String::from("Halo");
    let s2 = "Dunia";

    cetak(&s1); // String bisa dipinjam sebagai &str
    cetak(s2);  // &str langsung
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

pub fn demo_lifetime() {
    let a = String::from("panjang");
    let b = "pendek";
    let result = longest(&a, b);
    println!("Yang paling panjang: {}", result);
}

// Fungsi-fungsi yang dipanggil di main.rs
pub fn demo_conditional() {
    let angka = 7;
    
    if angka < 5 {
        println!("Angka kurang dari 5");
    } else if angka == 5 {
        println!("Angka sama dengan 5");
    } else {
        println!("Angka lebih dari 5");
    }
}

pub fn demo_loops() {
    // Loop for
    for i in 1..5 {
        println!("For loop: {}", i);
    }
    
    // Loop while
    let mut counter = 0;
    while counter < 3 {
        println!("While loop: {}", counter);
        counter += 1;
    }
}

pub fn greet(nama: &str) {
    println!("Halo, {}!", nama);
}

pub fn demo_shadowing() {
    let x = 5;
    println!("Nilai awal x: {}", x);
    
    let x = x + 1;
    println!("Setelah shadowing pertama: {}", x);
    
    let x = x * 2;
    println!("Setelah shadowing kedua: {}", x);
}
