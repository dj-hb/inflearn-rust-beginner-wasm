// // 1. 문자열 입력
// use std::io;
// fn main() {
//     println!("문자열을 입력해주세요!");
//     let mut whatyousay = String::new();
//     io::stdin().read_line(&mut whatyousay).expect("입력실패"); // expect 는 언제 실행되는지?
//     println!("당신이 입력한 문자열: {}", whatyousay);
// }


// // 2. 숫자 입력
// use std::io;
// fn main() {
//     println!("숫자를 입력해주세요!");
//     let mut whatyousay = String::new();
//     io::stdin().read_line(&mut whatyousay).expect("입력실패");
//     let whatyousay: i32 = whatyousay.trim().parse().expect("입력값이 숫자가 아닙니다.");
//     println!("당신이 입력한 숫자: {}", whatyousay);
// }


// 3. 데이터 타입
fn main() {
    let tenz = [10; 5];
    let last = tenz[4];
    println!("last: {last}");
    println!("tenz: {:?}", tenz)
}