// Scalar

// fn main() {
//     let add = 3 + 8;
//     let sub = 22.2 - 4.5;
//     let mul = 3 * 5;
//     let quotient = 12.3 / 3.14;
//     let truncated = 7 / 4;
//     let remainder = 7 % 4;

//     println!("add: {}", add);
//     println!("sub: {}", sub);
//     println!("mul: {}", mul);
//     println!("quotient: {}", quotient);
//     println!("truncated: {}", truncated);
//     println!("remainder: {}", remainder);

//     let t = true;
//     let f: bool = false;

//     println!("t: {}", t);
//     println!("f: {}", f);

//     let c = 'A';
//     let z: char = '한';
//     // let z: char = '한빈';
//     let unicorn = '🦄';

//     println!("c: {}", c);
//     println!("z: {}", z);
//     println!("unicorn: {}", unicorn);
// }

// Compound

// Tuple

// fn main() {
//     // let t: (i32, bool, f64) = (32, true, 1.14);
//     // let (x, y, z) = t;
//     // println!("t: {:?}", t);
//     // println!("y: {y}");


//     // let t: (i32, bool, f64) = (32, true, 1.14);
//     // let x = t.0;
//     // let y = t.1;
//     // let z = t.2;
//     // // let zz = t.3;
//     // println!("x: {}", x);
//     // println!("y: {}", y);
//     // println!("z: {}", z);

//     // 뭐지이건
//     // let unit: () = ();
//     // println!("unit: {:?}", unit);

// }

// Array

fn main() {
    // // let x = [1, 2, 3, 4, 5];
    // // let x = [1, 2, 3, 4, 5, "hey"];
    // // let x = (1, 2, 3, 4, 5, "hey");

    // let x: [i32; 5] = [1, 2, 3, 4, 5];
    // // let x: [i32; 5] = [1, 2, 3, 4, 5, 6];
    // let a = x[0];
    // println!("a: {a}");



    let threes = [3; 100];
    let last = threes[99];
    println!("last: {last}");

    let hellos = ["안녕"; 10];
    println!("hellos: {:?}", hellos); // 포맷팅??
}