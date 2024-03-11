// 1. 변수 let, mut

// fn main() {
//     // let x = 5;
//     // println!("x의 값: {x}");
//     // x = 6; // 에러 발생
//     // println!("x의 값: {x}");

//     // let mut y = 5;
//     // y = 6;
//     // println!("y의 값: {y}");

// }



// 2. 상수 const

// const PI: f32 = 3.141592;

// fn main() {
//     println!("PI 상수값은 {PI}입니다.")
// }



// 3. 변수 가리기 shadowing

fn main() {
    let x = 3; // 얘는 shadowing 된다. (사라지는 게 아니라, 가려지는 것이다. 이 x랑 밑에 x는 다른 변수이다.)
    println!("x의 값: {x}");
    let x = x + 1;
    println!("x의 값: {x}");
    {
        let x = x * 2;
        println!("안쪽 범위(스코프)에서 x의 값: {x}");
    }
    println!("x의 값: {x}");
}