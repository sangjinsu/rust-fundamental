fn main() {
    // 리터럴 문자열 조합
    println!("{} {}", "Hello,", "world!");

    // 줄바꿈
    println!("Hello,\nworld!");

    // 탭
    println!("Hello,\tworld!");

    // 여러 줄 출력
    println!("Hello,");
    println!("world!");

    // 정수 출력
    println!("{} days", 31);

    // 변수 사용
    let days = 31;
    println!("{} days", days);

    // 실수 출력
    println!("{} days", 31.0);

    // 2진수 출력
    println!("{:b} days", 31);

    // _ 사용
    println!("{} dollars", 31_000);

    // 주석 예시 3가지
    // 이 줄은 주석입니다.
    // println!("Hello, world!"); // 이 줄은 주석입니다.
    // println!("Hello, world!"); /* 이 줄은 주석입니다. */

}


// doc 주석
/// 이 주석은 문서화 주석입니다.
/// 이 주석은 문서화 주석입니다.
fn add(a: i32, b: i32) -> i32 {
    a + b
}
