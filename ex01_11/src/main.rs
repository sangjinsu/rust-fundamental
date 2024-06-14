fn main() {
    let mut menu = vec![
        "짜장면",
        "짬뽕",
        "볶음밥",
        "탕수육",
        "군만두",
    ];

    for (index, item) in menu.iter().enumerate() {
        println!("{}: {}", index, item);
    }

    let mut order = vec![];
    order.push(menu[0]);
    order.push(menu[2]);
    order.push(menu[4]);

    for item in order.iter() {
        println!("{}", item);
    }

    // vector reverse
    order.reverse();
    for item in order.iter() {
        println!("{}", item);
    }

    // vector sort
    order.sort();
    for item in order.iter() {
        println!("{}", item);
    }

    vec![1, 2, 3, 4, 5]
        .iter()
        .for_each(|x| println!("{}", x));


    // 메뉴 추가
    menu.push("깐풍기");
    menu.push("탕짜면");

    for (index, item) in menu.iter().enumerate() {
        println!("{}: {}", index, item);
    }

    // 메뉴 삭제
    menu.remove(2);
    menu.remove(3);

    for (index, item) in menu.iter().enumerate() {
        println!("{}: {}", index, item);
    }

    // 메뉴 수정
    menu[0] = "짜장면(볶음)";
    menu[1] = "짬뽕(군만두)";

    for (index, item) in menu.iter().enumerate() {
        println!("{}: {}", index, item);
    }

    // 벡터 map

    let menu = vec![
        "짜장면",
        "짬뽕",
        "볶음밥",
        "탕수육",
        "군만두",
    ];

    let menu: Vec<String> =
        menu.
            iter().
            map(|x| x.to_string()).collect();

    for item in menu.iter() {
        println!("{}", item);
    }

    // vector slice
    let menu = vec![
        "짜장면",
        "짬뽕",
        "볶음밥",
        "탕수육",
        "군만두",
    ];

    let slice = &menu[1..4];
    for item in slice.iter() {
        println!("{}", item);
    }

    // 중간 값 삭제
    let mut menu = vec![
        "짜장면",
        "짬뽕",
        "볶음밥",
        "탕수육",
        "군만두",
    ];

    menu.splice(1..4, vec!["깐풍기", "탕짜면"]);

    for item in menu.iter() {
        println!("{}", item);
    }
}
