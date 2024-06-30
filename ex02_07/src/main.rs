// 정적 할당
// - 컴파일 시점에 메모리를 할당하는 방식
// - 메모리 할당이 고정적이기 때문에 메모리를 동적으로 할당할 수 없다.

static G_VALUE: i32 = 10;
static G_VALUE2: i32 = 20;
static mut G_VALUE3: i32 = 30;
// 스택 할당
// - 함수 호출 시 사용되는 메모리
// - 함수 호출 시 메모리를 할당하고 함수 종료 시 메모리를 해제한다.
// - 함수 호출 시 사용되는 메모리를 스택 프레임이라고 한다.

// 힙 할당
// - 동적으로 메모리를 할당하는 방식
// - 메모리를 할당하고 해제하는 것은 개발자가 직접 해야 한다.

// 레지스터 할당
// - CPU 내부에 있는 메모리
// - CPU 내부에 있는 메모리를 사용하기 때문에 속도가 빠르다.


// 메모리 할당 종류
// 1. 정적 할당
// 2. 스택 할당
// 3. 힙 할당
// 4. 레지스터 할당 => CPU 내부에 있는 메모리
// malloc, new
fn main() {
    println!("{} {}", G_VALUE, G_VALUE2);
    unsafe {
        G_VALUE3 = 40;
        println!("{}", G_VALUE3);
    }

    unsafe { f1(); }
    unsafe { f1(); }
    unsafe { f1(); }
}

unsafe fn f1 () {
    static mut G_VALUE4: i32 = 10;
    G_VALUE4 += 50;
    if G_VALUE4 > 100 {
        G_VALUE4 = 0;
    }

    println!("{}", G_VALUE4);
}
