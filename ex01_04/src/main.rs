fn main() {
    // 실수 연산

    // 덧셈
    println!("1 + 2 = {}", 1.0 + 2.0);

    // 뺄셈
    println!("1 - 2 = {}", 1.0 - 2.0);

    // 곱셈
    println!("1 * 2 = {}", 1.0 * 2.0);

    // 나눗셈
    println!("1 / 2 = {}", 1.0 / 2.0);

    // 나머지
    println!("1 % 2 = {}", 1.0 % 2.0);

    // 실수 연산
    println!("1.9 + 2.2 = {}", 1.9 + 2.2);
    println!("1.9 + 2.3 = {}", 1.9 + 2.3);
    // 1.9 + 2.3 = 4.199999999999999


    // 상수

    // PI
    println!("PI = {}", std::f64::consts::PI);

    // E
    // 자연상수
    println!("E = {}", std::f64::consts::E);

    // INFINITY
    // 무한대
    println!("INFINITY = {}", f64::INFINITY);

    // NEG_INFINITY
    // 음의 무한대
    println!("NEG_INFINITY = {}", f64::NEG_INFINITY);

    // NAN
    // Not a Number
    println!("NAN = {}", f64::NAN);

    // EPSILON
    // 1과 1.0 사이의 차이
    println!("EPSILON = {}", f64::EPSILON);

    // MIN
    // f64의 최소값
    println!("MIN = {}", f64::MIN);

    // MAX
    // f64의 최대값
    println!("MAX = {}", f64::MAX);

    // MIN_POSITIVE
    // f64의 최소 양수값
    println!("MIN_POSITIVE = {}", f64::MIN_POSITIVE);

    // MIN_EXP
    // f64의 최소 지수값
    println!("MIN_EXP = {}", f64::MIN_EXP);

    // MAX_EXP
    // f64의 최대 지수값
    println!("MAX_EXP = {}", f64::MAX_EXP);

    // 리터럴 붙이기
    println!("hello \
             world \
             ha ha ha ha");
}
