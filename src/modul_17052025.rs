pub struct User {
    pub username: String,
    pub email: String,
    pub active: bool,
}

pub fn demo_struct_user() {
    let user1 = User {
        username: String::from("nuha"),
        email: String::from("nuha@example.com"),
        active: true,
    };

    println!("Username: {}", user1.username);
}


pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // method
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    // constructor
    pub fn new(w: u32, h: u32) -> Self {
        Self { width: w, height: h }
    }
}

pub fn demo_struct_rectangle() {
    let rect = Rectangle::new(10, 5);
    println!("Area: {}", rect.area());
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub fn demo_enum_direction() {
    let arah = Direction::North;

    match arah {
        Direction::North => println!("Ke utara"),
        Direction::South => println!("Ke selatan"),
        Direction::East  => println!("Ke timur"),
        Direction::West  => println!("Ke barat"),
    }
}

pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

pub fn demo_enum_message() {
    let pesan = Message::Move { x: 5, y: 8 };

    match pesan {
        Message::Quit => println!("Keluar"),
        Message::Move { x, y } => println!("Pindah ke: {}, {}", x, y),
        Message::Write(text) => println!("Menulis: {}", text),
    }
}


pub fn bagi(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

pub fn demo_option() {
    match bagi(10, 2) {
        Some(hasil) => println!("Hasil: {}", hasil),
        None => println!("Tidak bisa dibagi 0"),
    }
}

pub fn baca_file(nama: &str) -> Result<String, String> {
    if nama == "config.txt" {
        Ok(String::from("data file"))
    } else {
        Err(String::from("File tidak ditemukan"))
    }
}

pub fn demo_result() {
    match baca_file("config.txt") {
        Ok(data) => println!("Isi: {}", data),
        Err(e) => println!("Error: {}", e),
    }
}
