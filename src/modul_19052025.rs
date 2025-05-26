use std::collections::HashMap;

pub fn demo_collections() {
    // Vec
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("Isi Vec: {:?}", numbers);

    // HashMap
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Alice", 90);
    scores.insert("Bob", 80);
    println!("Nilai Alice: {:?}", scores.get("Alice"));

    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}


use std::fs::File;
use std::io::{self, Read};

pub fn baca_file(nama_file: &str) -> Result<String, io::Error> {
    let mut file = File::open(nama_file)?; // jika gagal, langsung return Err
    let mut isi = String::new();
    file.read_to_string(&mut isi)?;
    Ok(isi)
}

pub fn demo_file_io() {
    match baca_file("contoh.txt") {
        Ok(data) => println!("Isi file:\n{}", data),
        Err(e) => println!("Terjadi error: {}", e),
    }
}

pub fn pembagian(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Tidak bisa membagi dengan nol!");
    }
    a / b
}

pub fn bagi(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Tidak bisa membagi dengan nol".to_string())
    } else {
        Ok(a / b)
    }
}

pub fn demo_error_handling() {
    // panic (unrecoverable error)
    // let hasil = pembagian(10, 0); // Ini akan panic
    // println!("Hasil: {}", hasil);

    // recoverable error dengan Result
    match bagi(10, 0) {
        Ok(hasil) => println!("Hasil: {}", hasil),
        Err(e) => println!("Terjadi kesalahan: {}", e),
    }
}
