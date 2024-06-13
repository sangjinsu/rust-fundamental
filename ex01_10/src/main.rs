fn main() {
    // 변수의 범위
    // 변수가 보여지는 범위는 변수가 선언된 블록 내부
    let x = 10;
    {
        let y = 20;
        let x = 30;
        println!("x = {}, y = {}", x, y);
    }
    // println!("x = {}, y = {}", x, y); // error: cannot find value `y` in this scope
    println!("x = {}", x);


    // 배열
    let a = [1, 2, 3, 4, 5];
    for i in 0..5 {
        println!("a[{}] = {}", i, a[i]);
    }
    println!("a.len() = {}", a.len());

    // 벡터
    let mut v = vec![1, 2, 3, 4, 5];
    for i in 0..5 {
        println!("v[{}] = {}", i, v[i]);
    }
    println!("v.len() = {}", v.len());
    v.push(6);
    println!("v = {:?}", v);

    let value = v.pop().unwrap();
    println!("value = {:?}", value);

    v.pop();
    v.pop();
    v.pop();
    v.pop();
    v.pop();
    v.pop();
    println!("v = {:?}", v);

    let mut v = vec![1, 2, 3, 4, 5];
    loop {
        match v.pop() {
            Some(value) => {
                println!("value = {:?}", value);
            },
            None => {
                println!("None");
                break;
            },
        }
    }

    // 패닉 발생
    // let mut v = vec![1, 2, 3, 4, 5];
    // loop {
    //     let value = v.pop().unwrap();
    //     println!("value = {:?}", value);
    // }

    // 배열 크기 명시적 지정
    let a = [0; 5];
    for i in 0..5 {
        println!("a[{}] = {}", i, a[i]);
    }

    // 벡터 크기 명시적 지정
    let v = vec![0; 5];
    for i in 0..5 {
        println!("v[{}] = {}", i, v[i]);
    }

    // 문자열 벡터 명시적 지정
    let v = vec!["abc"; 5];
    for i in 0..5 {
        println!("v[{}] = {}", i, v[i]);
    }

    // 피보나치 수열 20개
    let mut fib = vec![0; 20];
    fib[1] = 1;
    for i in 2..20 {
        fib[i] = fib[i - 1] + fib[i - 2];
    }
    println!("fib = {:?}", fib);


    // 다차원 배열
    let a = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    for i in 0..3 {
        for j in 0..3 {
            println!("a[{}][{}] = {}", i, j, a[i][j]);
        }
    }

    // 다차원 벡터
    let mut v = vec![vec![0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            v[i][j] = i * 3 + j + 1;
        }
    }
    println!("v = {:?}", v);
}
