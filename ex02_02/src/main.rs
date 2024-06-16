fn main() {
    println!("Hello, world!");

    // double
    let x = 10;
    let y = double(x);
    println!("y = {}", y);

    // non_return
    let non = non_return();
    println!("non = {:?}", non);
    // non_return 함수는 리턴 값이 없기 때문에 () 빈 튜플로 리턴됨

    // early_return
    let z = early_return(5);
    println!("z = {}", z);

    // early_return2
    let w = early_return2(5);
    println!("w = {}", w);

    // early_return3
    let v = early_return3(5);
    println!("v = {}", v);

    // direct_value
    let u = direct_value();
    println!("u = {}", u);

    // multiple_return
    let (a, b) = multiple_return(5);
    println!("a = {}, b = {}", a, b);

    // return_struct
    let result = return_struct();
    match result {
        Ok(i) => println!("i = {}", i),
        Err(e) => println!("e = {}", e),
    }

    // return_enum
    let result = return_enum();
    match result {
        Ok(i) => {
            match i {
                MyEnum::A(s) => println!("s = {}", s),
            }
        }
        Err(e) => println!("e = {}", e),
    }
}


fn double(x: i32) -> i32 {
    x * 2
}

// 리턴 값이 없는 경우
fn non_return() {
    println!("non_return");
}

// 조기 반환 함수
fn early_return(x: i32) -> i32 {
    if x < 10 {
        return 0;
    }
    x
}

// 조기 반환 함수 에러 케이스
fn early_return2(x: i32) -> i32 {
    if x < 10 {
        0; // 밑으로 실행, return 꼭 필요
    }
    x
}

// 조기 반환 함수 2
fn early_return3(x: i32) -> i32 {
    if x < 10 {
        return 0;
    }
    x
}


// 바로 반환 함수
fn direct_value() -> i32 {
    10
}

/// 두 개의 값을 반환하는 함수
/// # Arguments
/// * `x` - i32 타입의 정수
/// # Returns
/// * i32 - x * 2
/// * i32 - x * 3
/// # Examples
/// ```
/// let (a, b) = multiple_return(5);
/// println!("a = {}, b = {}", a, b);
/// ```
/// 출력 결과: a = 10, b = 15
fn multiple_return(x: i32) -> (i32, i32) {
    (x * 2, x * 3)
}

/// 구조체 형태 반환 값
/// # Returns
/// * Result<i32, String> - 정상적인 경우 i32 값
/// * Result<i32, String> - 에러인 경우 에러 메시지
/// # Examples
/// ```
/// let result = return_struct();
/// match result {
///    Ok(i) => println!("i = {}", i),
///   Err(e) => println!("e = {}", e),
/// }
/// ```
/// 출력 결과: i = 10
fn return_struct() -> Result<i32, String> {
    Ok(10)
}



// enum 예시
enum MyEnum {
    A(String),
}

// enum 반환 함수
fn return_enum() -> Result<MyEnum, String> {
    Ok(MyEnum::A("Hello".to_string()))
}
