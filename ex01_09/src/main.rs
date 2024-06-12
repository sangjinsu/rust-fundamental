fn main() {
    // 1 ~ 10 까지의 제곱 출력
    for i in 1..11 {
        println!("{}^2 = {}", i, i * i);
    }

    // 1 ~ 10 까지의 5승 출력
    for i in 1..11 {
        println!("{}^5 = {}", i, i32::pow(i, 5));
    }

    // while 문
    let mut i = 0;
    while i < 10 {
        println!("i = {}", i);
        i += 1;
    }

    // 1 ~ 50 까지 정수 중 3으로 나누어 떨어지지 않는 수의 제곱 중 400 이하 수 출력
    for i in 1..51 {
        if i % 3 != 0 {
            let square = i * i;
            if square <= 400 {
                println!("{}^2 = {}", i, square);
            }
        }
    }

    // while 문
    let mut i = 1;
    while i < 51 {
        if i % 3 != 0 {
            let square = i * i;
            if square <= 400 {
                println!("{}^2 = {}", i, square);
            }
        }
        i += 1;
    }

    // continue 문
    for i in 1..51 {
        if i % 3 == 0 {
            continue;
        }
        let square = i * i;
        if square <= 400 {
            println!("{}^2 = {}", i, square);
        }
    }

    // 400 이상 수가 나오면 종료
    for i in 1..51 {
        if i % 3 == 0 {
            continue;
        }
        let square = i * i;
        if square > 400 {
            break;
        }
        println!("{}^2 = {}", i, square);
    }

    // loop 문 무한 루프
    let mut i = 1;
    loop {
        if i % 3 == 0 {
            i += 1;
            continue;
        }
        let square = i * i;
        if square > 400 {
            break;
        }
        println!("{}^2 = {}", i, square);
        i += 1;
    }

    // loop 문
    let mut i = 1;
    loop {
        if i % 3 == 0 {
            i += 1;
            continue;
        }
        let square = i * i;
        if square > 400 {
            break;
        }
        println!("{}^2 = {}", i, square);
        i += 1;
    }

    // 2중 for 문
    for i in 1..6 {
        for j in 1..6 {
            println!("{} * {} = {}", i, j, i * j);
        }
    }

    // 범위 고정 예시
    // 범위는 고정이 아니라 변수로 사용 가능
    let mut limit = 5;
    for n in 1..=limit + 3 {
        limit -= 1;
        println!("n = {}", n);
        println!("limit = {}", limit);
    }
}
