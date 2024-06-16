fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    f1(&mut arr);

    println!("arr = {:?}", arr);

    let arr = [1, 2, 3, 4, 5];
    let new_arr = f2(arr);
    println!("new_arr = {:?}", new_arr);
    println!("arr = {:?}", arr);

    let a = 10;
    let b = &a;
    let c = &b;
    let d = &c;
    println!("a = {}", a);
    println!("b = {}", b);
    println!("*b = {}", *b);
    println!("c = {}", c);
    println!("*c = {}", *c);
    println!("d = {}", d);
    println!("*d = {}", *d);
}

// 배열 포인터 참조 파라미터 함수
fn f1(arr: &mut [i32; 5]) {
    for i in 0..5 {
        arr[i] *= 2;
    }
}

// 배열 값 +1 함수
fn f2(arr: [i32; 5]) -> [i32; 5] {
    let mut new_arr = arr;
    for i in 0..5 {
        new_arr[i] += 1;
    }
    new_arr
}