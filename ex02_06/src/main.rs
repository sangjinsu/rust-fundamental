// generic divide function
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("Divide by zero".to_string());
    }
    return Ok(a / b);
}


fn show_divide(a: f64, b: f64) {
    let result = divide(a, b);
    if result.is_ok() {
        println!("result = {}", result.unwrap());
    } else if result.is_err() {
        println!("error = {}", result.unwrap_err());
    } else {
        println!("unknown error");
    }
}

fn main() {
    println!("Hello, world!");

    show_divide(10.0, 20.0);
    show_divide(10.0, 0.0);

    // unwrap
    let result = divide(10.0, 20.0);
    println!("result = {}", result.unwrap());

    if let Ok(result) = divide(10.0, 20.0) {
        println!("result = {}", result);
    }

    if let Ok(result) = divide(10.0, 0.0) {
        println!("result = {}", result);
    }

    if let Err(error) = divide(10.0, 0.0) {
        println!("error = {}", error);
    }

    let mut k = Some(10);
    match k {
        Some(ref mut x) => *x = 20,
        None => (),
    }

    // is_some
    let k = Some(10);
    if k.is_some() {
        println!("k = {}", k.unwrap());
    }

    // is_none
    let k : Option<isize> = None;
    if k.is_none() {
        println!("k is None");
    }

    // unwrap_or
    let k = Some(10);
    println!("k = {}", k.unwrap_or(0));

    let mut v = vec![1, 2, 3, 4, 5];
    for _ in 0..9 {
        let r = v.pop();
        if r.is_none() {
            break;
        }
        println!("r = {}", r.unwrap());
    }
}
