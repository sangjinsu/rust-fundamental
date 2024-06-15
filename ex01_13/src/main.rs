fn main() {
    // 자료형
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


    let mut l;
    l = "abcd".len(); // 4
    println!("l = {}", l);
    l = "가나다라".len(); // 12, 바이트 수
    println!("l = {}", l);
    l = "가나다라".chars().count(); // 4, 글자 수
    println!("l = {}", l);

    // 이진수 표현
    let b = 0b1101_1010;
    println!("b = {}", b);

    // 16진수 표현
    let h = 0x1A;
    println!("h = {}", h);

    // 8진수 표현
    let o = 0o32;
    println!("o = {}", o);

    // 바이트
    let byte = b'A';
    println!("byte = {}", byte);

    // 바이트 문자열
    let byte_str = b"Hello";
    println!("byte_str = {:?}", byte_str);

    // 유니코드 스트링
    let unicode_str = "안녕하세요";
    println!("unicode_str = {}", unicode_str);

    // eprintln 매크로
    eprintln!("error message");

    // 2진수 16진수
    let bin = 0b1101_1010;
    println!("bin = {}", bin);
    let hex = 0x1A;
    println!("hex = {}", hex);


    // 문자
    let e = 'A';
    let f: char = 'B';
    println!("e = {}", e);
    println!("f = {}", f);

    // 문자열
    let g = "Hello, world!";
    let h: &str = "Hello, Rust!";
    println!("g = {}", g);
    println!("h = {}", h);


    // 부호 있는 정수
    // i8, i16, i32, i64, i128, isize
    let _a: i8 = 10;
    let _b: i16 = 20;
    let _c: i32 = 30;
    let _d: i64 = 40;
    let _e: i128 = 50;
    let _f: isize = 60;

    // 부호 없는 정수
    // u8, u16, u32, u64, u128, usize
    let _a: u8 = 10;
    let _b: u16 = 20;
    let _c: u32 = 30;
    let _d: u64 = 40;
    let _e: u128 = 50;
    let _f: usize = 60;

    // 부동 소수점
    // f32, f64
    let _a: f32 = 10.0;
    let _b: f64 = 20.0;

    // 불리언
    let _a: bool = true;
    let _b: bool = false;

    // 문자
    let _a: char = 'A';

    // 문자열
    let _a: &str = "Hello, world!";

    // 바이트
    let _a: u8 = b'A';

    // 바이트 문자열
    let _a: &[u8; 5] = b"Hello";

    // 유니코드 스트링
    let _a: &str = "안녕하세요";

    // 형변환
    let a: i32 = 10;
    let _b: i64 = a as i64;
    println!("b = {}", _b);

    // 형변환
    let a: i32 = 10;
    let b: i64 = a.into();
    println!("b = {}", b);

    // 형변환
    let a: i32 = 10;
    let b: i64 = i64::from(a);
    println!("b = {}", b);

    // 형변환
    let a: i32 = 10;
    let b: i64 = a.try_into().unwrap();
    println!("b = {}", b);

    // 소문자 전체 출력
    for c in 'a'..='z' {
        print!("{}", c);
    }

    // 대문자 전체 출력
    for c in 'A'..='Z' {
        print!("{}", c);
    }

    // 튜플
    let a = (10, 20);
    println!("a = {:?}", a);

    // 빈 튜플
    let a = ();
    println!("a = {:?}", a);

    // if, while 튜플
    let a = if false {};
    println!("a = {:?}", a);

    let b = while false {};
    println!("b = {:?}", b);

    // 정수 상수
    const MAX: i32 = 100;
    println!("MAX = {}", MAX);

    // 부동 소수점 상수
    const FLOAT: f64 = 3.14;
    println!("PI = {}", FLOAT);

}
