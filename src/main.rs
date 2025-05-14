fn main() {
    println!("Hello, world!");

    let x = 5; // immutable nilainya tidak bisa diubah setelah didefinisikan
    let mut y = 10; // mutable Mutable Variable (Bisa Diubah)
    println!("x: {}, y: {}", x, y);

    // tipe data primiive
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let arr = [1, 2, 3, 4, 5];
    println!("tup: {:?}", tup);
    println!("arr: {:?}", arr);
}
