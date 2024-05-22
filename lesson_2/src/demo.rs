fn demo() {
    println!("Hello, world!");

    let a: u8 = 32;
    let b: i32 = -5;
    let result: bool = demo(a, b);

    let mut counter = 0;

    let result_2 = loop {

        if counter == 10 {
             break 42;
        }

        counter += 1;
       
    };

    while counter < 100 {
        counter += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for el in a {
        print!("The value {el}")
    }
}

// Statement - Anweisungen, die Aktionen ausführen und keinen Wert zurückgeben
// Expression - Anweisungen, die einen Wert berechnen und zurückgeben
fn demo(a: u8, b: i32) -> bool  {
    println!("Hello, world {a}!");

    if a > 10 {
        true
    } else if b > 100 {
        false
    } else {
        true
    }
}