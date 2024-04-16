// fn main() {
//     // println!("Hello, world!");

//     // 1. 단순 데이터는 복사 (Stack 에서)
//     // {
//     //     let x = 3; // stack 에 저장
//     //     let y = x; // x 의 값을 복사해서 y 에 저장
//     //     println!("x: {x}, y: {y}");
//     // }

//     // // 2. 문자열 String 은 Heap 메모리에 위치
//     // {
//     //     let s1 = String::from("Hello"); // 힙에 위치
//     //     println!("s1: {s1}");
//     //     let s2 = s1; // 소유권이 s1에서 s2로 이동
//     //     println!("s2: {s2}");
//     //     // println!("s1: {s1}"); // 에러 발생, 소유권을 잃어버린 변수는 접근 불가 (s1)
//     // }
//     // // 블럭이 끝나면서 s2 에 대해서 메모리 해제 작업이 일어난다. (소유권을 가지고 있기 때문에)

//     // // 3. 복사해서 쓰고 싶다면 어떻게??
//     // {
//     //     let s1 = String::from("Hello");
//     //     let s2 = s1.clone(); // 힙 메모리 자체도 복사가 된다. s1 이랑 s2 둘 다 각각 소유권을 가지고 있다.
//     //     println!("s2: {s2}, s1: {s1}");
//     // }

// }



// // 4. 함수 호출 시 소유권 이동
// fn main() {
//     let s = String::from("헬로"); // 결과는 6이 나올 것 ... 한글을 utf-8로 인코딩하면 한 글자당 3바이트
//     string_length(s); // 함수 string_length 로 소유권이 넘어간다.
//     // println!("s는 {}", s); // 에러 발생, 소유권을 잃어버린 변수는 접근 불가 (s)

//     // 하지만 기본 데이터 타입은 소유권이 적용되지 않는다.
//     let x = 5;
//     double(x);
//     println!("x는 {}", x); // 에러 발생하지 않는다.

// }

// fn string_length(s: String) {
//     println!("문자열 s의 길이는 {}", s.len());
// }

// fn double(x: i32) {
//     println!("x는 {}", x);
// }





// // 5. 함수 반환값의 소유권 이동
// fn main() {
//     let s1 = String::from("헬로");
//     let s2 = check_string_length(s1); // 함수로 소유권 이동, 함수로부터 반환 받은 값의 소유권을 s2 가 가지게 된다.
//     println!("s2는 {}", s2);
//     // println!("s1는 {}", s1); // 에러 발생
// }

// fn check_string_length(s: String) -> String {
//     println!("문자열 s의 길이는 {}", s.len());
//     s // 소유권을 반환
// }



// 6. 불필요한 소유권 이동 문제

// fn main() {
//     let s = String::from("헬로");
//     let s_length = string_length(s);
//     println!("s 길이는 {}", s_length);
//     // println!("s={}의 길이는 {}", s, s_length); // 에러 발생, s 의 소유권이 함수 string_length 로 이동되었기 때문에 접근 불가. 소유권을 다시 돌려받을 필요가 있다.
// }
// fn string_length(s: String) -> usize {
//     // println!("문자열 s의 길이는 {}", s.len());
//     s.len()
// }

fn main() {
    let s = String::from("헬로");
    let (s_length, s) = string_length(s);
    println!("s({})의 길이는 {}", s, s_length);
}
fn string_length(s: String) -> (usize, String) {
    (s.len(), s)
}

// 소유권이 넘어갔다가 다시 돌려받고 하는 과정이 번거로움
// 소유권 임대 !!
