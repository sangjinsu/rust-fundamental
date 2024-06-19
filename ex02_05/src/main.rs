// 제네릭 매커니즘
// 제네릭은 코드를 작성할 때 타입을 명시하지 않고 추상화된 타입을 사용할 수 있게 해준다.
// 제네릭은 함수, 구조체, 열거형, 메서드, 트레이트에 사용할 수 있다.
// 제네릭을 사용하면 코드의 중복을 줄일 수 있고, 코드의 유연성을 높일 수 있다.
// 제네릭을 사용하면 코드의 유지보수가 쉬워지고, 코드의 가독성을 높일 수 있다.
// 제네릭을 사용하면 코드의 성능을 향상시킬 수 있다.
// 제네릭을 사용하면 코드의 재사용성을 높일 수 있다.

fn swap<T>(a: T, b: T) -> (T, T) {
    return (b, a);
}

fn swap2<T1, T2>(a: T1, b: T2) -> (T2, T1) {
    return (b, a);
}

// 제네릭 열거형
// 열거형에 제네릭을 사용하면 열거형의 멤버에 제네릭을 사용할 수 있다.
enum Result1<T1, T2, T3> {
    Success(T1),
    Failure(T2, T3),
    Uncertain,
}

// Option<T> 제네릭 처리
// Option<T> 열거형은 제네릭을 사용하여 Some과 None의 타입을 지정할 수 있다.
// Null 값 처리를 위해 사용된다.


fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    return Some(a / b);
}

fn divide2(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Divide by zero".to_string());
    }
    return Ok(a / b);
}

fn main() {
    let a = 10;
    let b = 20;
    let (a, b) = swap(a, b);
    println!("a = {}, b = {}", a, b);

    let a = 'A';
    let b = 'B';
    let (a, b) = swap(a, b);
    println!("a = {}, b = {}", a, b);

    let a = 10.0;
    let b = 20.0;
    let (a, b) = swap(a, b);
    println!("a = {}, b = {}", a, b);

    let a = "Hello";
    let b = "World";
    let (a, b) = swap(a, b);
    println!("a = {}, b = {}", a, b);

    let a = 10;
    let b = 'A';
    let (a, b) = swap2(a, b);
    println!("a = {}, b = {}", a, b);

    Result1::<i32, i32, i32>::Failure(10, 20);
    Result1::<i32, i32, i32>::Success(10);

    let result: Result1<i32, i32, i32> = Result1::Failure(10, 20);
    match result {
        Result1::Success(i) => println!("i = {}", i),
        Result1::Failure(i, j) => println!("i = {}, j = {}", i, j),
        Result1::Uncertain => println!("Uncertain"),
    }

    let result: Result1<i32, i32, i32> = Result1::Success(10);
    match result {
        Result1::Success(i) => println!("i = {}", i),
        Result1::Failure(i, j) => println!("i = {}, j = {}", i, j),
        Result1::Uncertain => println!("Uncertain"),
    }

    let result: Result1<i32, i32, i32> = Result1::Uncertain;
    match result {
        Result1::Success(i) => println!("i = {}", i),
        Result1::Failure(i, j) => println!("i = {}, j = {}", i, j),
        Result1::Uncertain => println!("Uncertain"),
    }


    let a: Option<i32> = Some(10);
    match a {
        Some(i) => println!("i = {}", i),
        None => println!("None"),
    }

    // divide
    let a = 10;
    let b = 0;
    let result = divide(a, b);
    match result {
        Some(i) => println!("i = {}", i),
        None => println!("None"),
    }

    let a = 10;
    let b = 0;
    let result = divide2(a, b);
    match result {
        Ok(i) => println!("i = {}", i),
        Err(e) => println!("e = {}", e),
    }
}
