// 소유권 !! Ownership !!!!

fn main() {
    // 1.
    // let s: String = "헬로";
    // 이렇게 하면 에러 - "헬로"는 '문자열 리터럴' 타입이지, String 타입이 아니다.


    // 2.
    // let s: String = String::from("헬로");
    // String 이라는 namespace에 있는 from 함수를 사용해서 "헬로"라는 문자열 리터럴을 String 값으로 heap 에 위치시킨 다음 s 라는 이름을 붙이는 과정
    // println!("s는 {}", s);


    // 3.
    // String 은 mutable 하다. 하지만 s 는 immutable 하다. mutable 하게 하려면 mut 키워드를 사용해야 한다.
    // let mut s: String = String::from("헬로");
    // println!("s는 {}", s);
    // s.push_str(", 월드!");
    // println!("s는 {}", s);


    // 4.
    // 메모리에 할당한 문자열을 다 사용했으면? 다시 사용 가능한 메모리로 반환해야 한다.
    // 소유권 규칙...!
}
