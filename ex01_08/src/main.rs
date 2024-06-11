fn main() {
    // 조건문 예시
    let x = 10;
    if x > 5 {
        println!("x is greater than 5");
    } else {
        println!("x is less than or equal to 5");
    }

    // else if
    let y = 10;
    if y > 5 {
        println!("y is greater than 5");
    } else if y < 5 {
        println!("y is less than 5");
    } else {
        println!("y is equal to 5");
    }

    // 중첩 if
    let z = 10;
    if z > 5 {
        if z < 15 {
            println!("z is greater than 5 and less than 15");
        }
    }

    // if let
    // if let 은 패턴 매칭을 사용하여 Option<T> 타입을 처리할 때 사용
    let a = Some(10);
    if let Some(i) = a {
        println!("i = {}", i);
    }

    // Option
    let b = Some(10);
    match b {
        Some(i) => println!("i = {}", i),
        None => println!("None"),
    }

    // Option
    let c: Option<i32> = None;
    match c {
        Some(i) => println!("i = {}", i),
        None => println!("None"),
    }

    // Option
    let d = Some(10);
    if let Some(i) = d {
        println!("i = {}", i);
    } else {
        println!("None");
    }

    // Some 이란 이름을 사용하지 않고 _ 를 사용할 수도 있음
    let e = Some(10);
    if let Some(_) = e {
        println!("Some");
    } else {
        println!("None");
    }

    // Some 의 의미는 값이 있는 경우를 의미하고 None 은 값이 없는 경우를 의미
    // Some 은 값이 있는 경우에는 Some(값) 으로 표현하고 None 은 값이 없는 경우에는 None 으로 표현
    // Option<T> 는 값이 있을 수도 있고 없을 수도 있는 경우에 사용하는 열거형
    // Option<T> 의 값은 Some(T) 또는 None 중 하나

    // match 는 패턴 매칭을 사용하여 Option<T> 타입을 처리할 때 사용
    // 2 중 match
    let f = Some(10);
    match f {
        Some(i) => match i {
            0 => println!("zero"),
            _ => println!("non-zero"),
        },
        None => println!("None"),
    }

    // if 문 표현식
    let g = if true { 10 } else { 20 };
    println!("g = {}", g);
}
