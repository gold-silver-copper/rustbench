fn main() {
    struct FancyNum {
        num: u8,
    }

    let mut fancy_num = FancyNum { num: 5 };
    let moved_num = fancy_num;
    fancy_num = FancyNum { num: 6 };

    println!("Num: {}, Moved num: {}", fancy_num.num, moved_num.num);
}
