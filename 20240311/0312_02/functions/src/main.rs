// * 함수

// fn main() {
//     // println!("Hello, world!");
//     a_function();
//     print_number(5);
//     // print_number("asdf");
//     add_numbers(5, 10);
// }

// fn a_function() {
//     println!("함순데예");
// }

// fn print_number(x: i32) {
//     println!("x는 {x}입니다.");
// }

// fn add_numbers(a: i32, b: i32) {
//     let sum = a + b;
//     println!("a와 b의 합은 {sum}입니다.");
// }



// * 명령문 & 식

// fn main() {
//     // let y = (let x = 5); // 이렇게는 안 된다.

//     let y = {
//         let x = 5; // 명령문
//         // 마지막 줄 (식) - 블럭 전체의 반환값
//         // 7 // 식
//         3 + x // 식
//     };
//     println!("y는 {y}입니다.");
// }



// * 함수의 반환값 (Return Values)

fn main() {
    let a = circle_area(2.4);

    println!("반지름이 2.4인 원의 면적은 {a}입니다.");
}

const PI: f64 = 3.141592;

fn circle_area(radius: f64) -> f64 { // 반환값의 타입 지정 가능
    // PI * radius * radius; 명령문이 아닌 식(반환값)에는 세미콜론을 붙이지 않는다.
    PI * radius * radius
}
