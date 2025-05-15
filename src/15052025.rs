pub fn demo_conditional() {
    let number = 6;

    if number % 4 == 0 {
        println!("Kelipatan 4");
    } else if number % 3 == 0 {
        println!("Kelipatan 3");
    } else {
        println!("Bukan kelipatan 3 atau 4");
    }

    let result = match number {
        1 => "satu",
        2 => "dua",
        _ => "lainnya",
    };
    println!("Match result: {}", result);
}

pub fn demo_loops() {
    // loop
    let mut count = 0;
    loop {
        if count == 5 { break; }
        println!("looping ke-{}", count);
        count += 1;
    }

    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // for
    for number in (1..4).rev() {
        println!("{}", number);
    }
}

pub fn greet(name: &str) {
    println!("Halo, {}", name);
}

pub fn demo_shadowing() {
    let x = 5;

    {
        let x = x * 2; // shadowing
        println!("Dalam scope: {}", x);
    }

    println!("Luar scope: {}", x);
}
