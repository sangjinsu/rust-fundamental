// 함수 예시 코드를 보여줘

fn main() {
    println!("Hello, world!");
    another_function();

    f1();

    fn f1() {
        println!("f1");

        fn f2() {
            println!("f2");
        }

        f2();
    }

    // fn2(); // error: cannot find function `fn2` in this scope

    // 계산기 함수
    let a = 10;
    let b = 20;
    let c = '+';
    let result = calculator(a, b, c);
    println!("result = {}", result);

    // 제네릭 계산기 함수
    let a = 10;
    let b = 20;
    let c = '+';
    let result = generic_calculator(a, b, c);
    println!("result = {}", result);

    let a = 10.0;
    let b = 20.0;
    let c = '+';
    let result = generic_calculator(a, b, c);
    println!("result = {}", result);

    // 제네릭 필터 함수
    let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = filter(a, |x| x % 2 == 0);
    println!("result = {:?}", result);
}

fn another_function() {
    println!("Another function.");
}

// 계산기 함수
fn calculator(a: i32, b: i32, c: char) -> i32 {
    let result = match c {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => 0,
    };
    result
}

// 제네렉 계산기 함수
fn generic_calculator<T>(a: T, b: T, c: char) -> T
where
    T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::ops::Mul<Output=T> + std::ops::Div<Output=T> + Default + Copy,
{
    let result = match c {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => T::default(),
    };
    result
}

// 벡터 필터 제네릭 함수
fn filter<T: Copy>(a: Vec<T>, func: fn(T) -> bool) -> Vec<T> {
    let mut result = Vec::new();
    for i in a {
        if func(i.clone()) {
            result.push(i.clone());
        }
    }
    result
}

