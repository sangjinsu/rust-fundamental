fn main() {
    let x = 120;
    println!("x = {}", x);

    let x = "abc";
    println!("x = {}", x);

    let mut x = true;
    println!("x = {}", x);
    x = false;
    println!("x = {}", x);

    // 복합 할당 연산자
    let mut y = 10;
    println!("y = {}", y);
    y += 20;
    println!("y = {}", y);

    // 타입 변경
    let z = 100;
    println!("z = {}", z);
    let z = 100.0;
    println!("z = {}", z);

    let mut l ;
    l = "abcd".len(); // 4
    println!("l = {}", l);
    l = "가나다라".len(); // 12, 바이트 수
    println!("l = {}", l);
    l = "가나다라".chars().count(); // 4, 글자 수
    println!("l = {}", l);

    // 이모지 길이
    let emoji = "👍";
    println!("emoji.len() = {}", emoji.len()); // 4
    println!("emoji.chars().count() = {}", emoji.chars().count()); // 1
    // 다른 이모지 4개
    let emoji = "👍👎👌👊";
    println!("emoji.len() = {}", emoji.len()); // 16

    // 객체지향 문법 len 예시
    let s = String::from("가나다라");
    println!("s.len() = {}", s.len()); // 12

    // 절차지향 문법
    let s = "가나다라";
    println!("str::len(s) = {}", str::len(s)); // 12

}
