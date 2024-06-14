fn main() {
    // 배열 얕은 복사
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = arr1;
    println!("arr1 = {:?}", arr1);
    println!("arr2 = {:?}", arr2);

    // 벡터 얕은 복사
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec1;
    // println!("vec1 = {:?}", vec1); // error[E0382]: borrow of moved value: `vec1`
    println!("vec2 = {:?}", vec2);

    // 배열 깊은 복사
    let arr1 = [1, 2, 3, 4, 5];
    let mut arr2 = [0; 5];
    for i in 0..5 {
        arr2[i] = arr1[i];
    }

    println!("arr1 = {:?}", arr1);
    println!("arr2 = {:?}", arr2);

    // 벡터 깊은 복사

    let vec1 = vec![1, 2, 3, 4, 5];
    let mut vec2 = vec![0; 5];
    for i in 0..5 {
        vec2[i] = vec1[i];
    }

    println!("vec1 = {:?}", vec1);
    println!("vec2 = {:?}", vec2);

    // 벡터 깊은 복사
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec1.clone();
    println!("vec1 = {:?}", vec1);
    println!("vec2 = {:?}", vec2);

    // 벡터 깊은 복사
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec1.to_vec();
    println!("vec1 = {:?}", vec1);
    println!("vec2 = {:?}", vec2);

    // 벡터 깊은 복사
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec1.iter().map(|x| *x).collect::<Vec<i32>>();
    println!("vec1 = {:?}", vec1);
    println!("vec2 = {:?}", vec2);

    // 벡터 깊은 복사
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2: Vec<i32> = vec1.iter().map(|x| *x).collect();
    println!("vec1 = {:?}", vec1);
    println!("vec2 = {:?}", vec2);

    // 벡터 깊은 복사
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec1.iter().map(|x| *x).collect::<Vec<_>>();
    println!("vec1 = {:?}", vec1);
    println!("vec2 = {:?}", vec2);

}
