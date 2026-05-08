fn main() {
    let arr: [i32; 5] = [1, 23, 4, 5, 6];
    let arr1 = [3; 5];
    let mut arr2: [i32; 3] = [0, 0, 0];
    println!("{:?}", arr1);
    println!("{:?}", arr);
    println!("{:?}", arr2);

    arr2[0] = 1;
    arr2[1] = 3;
    arr2[2] = 2;

    println!("{:?}", arr2);
}
