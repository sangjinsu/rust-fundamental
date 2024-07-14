
// 스택 할당 
// - 함수 호출 시 사용되는 메모리
// - 함수 호출 시 메모리를 할당하고 함수 종료 시 메모리를 해제한다.
// - 함수 호출 시 사용되는 메모리를 스택 프레임이라고 한다.

// 스택 할당 제한 사항 
const SIZE : usize = 100_000;
const N_ARRAY :usize = 1_000_000;

fn create_array() -> [u8; SIZE] {
    [0u8;SIZE]
}

fn recursive_func(n: usize){
    let a = create_array();
    println!("{} {}", a[0], N_ARRAY - n + 1);
    if n > 1 {
        recursive_func(n - 1);
    }
}

fn main() {
    let a : u32 = 8;
    let b : i32 = 1_000_000;
    let c : f64 =   5.7;   
    let d : bool = true; 
    let e : char = 'A';
    
    println!("{} {} {} {} {}", a, b, c, d, e);
    println!("Hello, world!");

    recursive_func(N_ARRAY)

    // thread 'main' has overflowed its stack
    // fatal runtime error: stack overflow
    // Aborted (core dumped)
}
