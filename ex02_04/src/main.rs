fn main() {
    let a = 10;
    let b = square_root(a);
    println!("b = {}", b);

    let a = 10.0;
    let b = square_root(a);
    println!("b = {}", b);


    f2::<i32>('i', 10);
    f2::<char>('c', 'A');
    f2::<f64>('f', 10.0);

    let a = 10;
    let b = f3(a);
    println!("b = {}", b);

    let point = Point { x: 10, y: 20 };
    println!("point.x = {}", point.x());
    println!("point.y = {}", point.y());

    let point = Point { x: 10.0, y: 20.0 };
    println!("point.x = {}", point.x());
    println!("point.y = {}", point.y());

    let point = Point { x: 'A', y: 'B' };
    println!("point.x = {}", point.x());
    println!("point.y = {}", point.y());

    let square = Square { width: 10, height: 20 };
    println!("square.area = {}", square.area());

    let square = Square { width: 10.0, height: 20.0 };
    println!("square.area = {}", square.area());

}

// 제네릭
fn square_root<T>(x: T) -> T
where
    T: std::ops::Mul<Output=T> + std::ops::Div<Output=T> + std::ops::Add<Output=T> + Copy,
{
    x * x
}

// 제네릭 함수
fn f2<T: std::fmt::Display>(ch: char, x: T) {
    if ch == 'i' {
        println!("x = {}", x);
    } else {
        println!("ch = {}", ch);
    }
}

fn f3<T>(x: T) -> T {
    return x;
}


// 제네릭 구조체 예시
struct Point<T> {
    x: T,
    y: T,
}

// 제네릭 구조체 메소드

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T> Point<T> {
    fn y(&self) -> &T {
        &self.y
    }
}

// 제네렉 구조체 예시
struct Square<T> {
    width: T,
    height: T,
}

// 제네릭 구조체 메소드
impl<T> Square<T> {
    fn area(&self) -> T
    where
        T: std::ops::Mul<Output=T> + Copy,
    {
        self.width * self.height
    }
}





