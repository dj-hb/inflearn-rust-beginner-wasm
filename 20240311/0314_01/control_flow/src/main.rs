// * 조건문 (if, else if, else)

// fn main() {
//     let x = 7;

//     let condition = false;

//     let y = if condition { 3 } else { 5 };

//     println!("y의 값: {y}");

//     if x % 3 == 0 {
//         println!("x는 3으로 나누어 떨어집니다.");
//     } else if x % 3 == 1 {
//         println!("x를 3으로 나눈 나머지는 1입니다.");
//     } else {
//         println!("x를 3으로 나눈 나머지는 2입니다.");
//     }
// }

// * 반복문 (loop)

// fn main() {
//     // loop {
//     //     println!("반복");
//     // }


//     // let mut counter = 0;
//     // loop {
//     //     println!("반복");
//     //     counter += 1;
//     //     if counter == 5 {
//     //         break;
//     //     }
//     // }


//     // let mut counter = 0;
//     // let x = loop {
//     //     println!("반복");
//     //     counter += 1;
//     //     if counter == 5 {
//     //         break counter;
//     //     }
//     // };
//     // println!("x의 값: {x}");
// }

// * 반복문 (while)

// fn main() {
//     // let mut counter = 0;
//     // while counter < 4 {
//     //     println!("반복");
//     //     counter += 1;
//     // }


//     let xs = [10, 20, 30, 40, 50];
//     let mut idx = 0;
//     while idx < xs.len() {
//         println!("xs[{}] = {}", idx, xs[idx]);
//         idx += 1;
//     }
//     println!("종료")

// }

// * 반복문 (for)

fn main() {
    // let xs = [10, 20, 30, 40, 50];
    // for x in xs {
    //     println!("x = {}", x);
    // }
    // println!("종료")


    // for i in 0..7 {
    //     println!("i = {i}");
    // }


    // let xs = [10, 20, 30, 40, 50];
    // let l = xs.len();
    // for i in 0..l {
    //     println!("x = {}", xs[i]);
    // }


    let xs = [10, 20, 30, 40, 50];
    let l = xs.len();
    for i in (0..l).rev() {
        println!("x = {}", xs[i]);
    }
}