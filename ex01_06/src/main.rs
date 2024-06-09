fn main() {
    // 부울값
    let a = true;
    let b: bool = false;
    println!("a = {}", a);
    println!("b = {}", b);

    // 부울 연산
    println!("AND : {}", a && b);
    println!("OR : {}", a || b);
    println!("NOT : {}", !a);

    // 비교 연산
    let c = 10;
    let d = 20;
    println!("c == d : {}", c == d);
    println!("c != d : {}", c != d);
    println!("c > d : {}", c > d);
    println!("c < d : {}", c < d);
    println!("c >= d : {}", c >= d);
    println!("c <= d : {}", c <= d);

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

    // 문자열 결합
    let i = "Hello, ";
    let j = "world!";
    println!("{}{}", i, j);

    // 문자열 결합
    let k = "Hello, ";
    let l = "world!";
    let m = k.to_string() + l;
    println!("{}", m);

    // 문자열 비교
    let n = "aa";
    let o = "ba";
    println!("n == o : {}", n == o); // false
    println!("n != o : {}", n != o); // true
    println!("n > o : {}", n > o); // false
    println!("n < o : {}", n < o); // true
    println!("n >= o : {}", n >= o); // false
    println!("n <= o : {}", n <= o); // true

    // 대소문자 비교
    let p = "Hello";
    let q = "hello";
    println!("p == q : {}", p == q); // false
    println!("p.eq_ignore_ascii_case(q) : {}", p.eq_ignore_ascii_case(q)); // tru
    let aa = "A";
    let a = "a";
    println!("aa == a : {}", aa == a); // false
    println!("aa.eq_ignore_ascii_case(a) : {}", aa.eq_ignore_ascii_case(a)); // true
    println!("A > a : {}", aa > a); // false
    println!("A < a : {}", aa < a); // true
    println!("A >= a : {}", aa >= a); // false
    println!("A <= a : {}", aa <= a); // true

    // 대입 연산자의 타입 일관성
    let mut r = 10;
    println!("r = {}", r);
    r = 20;
    println!("r = {}", r);
    r += 10;
    println!("r = {}", r);
    r -= 5;
    println!("r = {}", r);

}
