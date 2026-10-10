fn main() {
    let mut value = 3;
    let value_cloned = value.clone();
    let borrow = &mut value;
    let _sum = value_cloned + 1;
    println!("{}", borrow);
}
