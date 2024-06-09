fn main() {
    // 합격 점수 : 80점 이상
    let score = 85;
    if score >= 80 {
        println!("합격입니다.");
    } else {
        println!("불합격입니다.");
    }

    // 타입
    let mut a = 10;
    println!("a = {}", a);

    // 타입 초기화
    let b: i32 = 20;

    // 변수 재사용
    a = 30;
    println!("a = {}", a);
    println!("b = {}", b);

    // 가변 변수
    let mut c = 40;
    println!("c = {}", c);
    c = 50;
    println!("c = {}", c);

    // 불변 변수
    let d = 60;
    println!("d = {}", d);
    // d = 70; // error[E0384]: cannot assign twice to immutable variable `d`

    // 지연된 초기화
    let e: i32;
    //  println!("e = {}", e); // error[E0381]: use of possibly-uninitialized variable: `e`
    e = 80;
    println!("e = {}", e);

    // let f;
    // println!("f = {:?}", f);
    // f = 90;

    let g;
    let mut g2 = 100;
    g = g2;
    println!("g = {:?}", g);
    g2 = 90;
    println!("g = {:?}", g);
    println!("g2 = {:?}", g2);

    // 언더바
    let _h = 110;
    // println!("h = {}", h); // error[E0425]: cannot find value `h` in this scope
    println!("h = {}", _h);
}
