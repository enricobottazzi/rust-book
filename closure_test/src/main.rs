fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut only_borrows = || {
        list = vec![4, 5, 6];
        println!("From closure: {:?}", list)
    };

    only_borrows();
    println!("Before calling closure: {:?}", list);
    println!("After calling closure: {:?}", list);
}