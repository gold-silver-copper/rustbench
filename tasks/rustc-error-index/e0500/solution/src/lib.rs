fn you_know_nothing(jon_snow: &mut i32) {
    let mut jon_copy = jon_snow.clone();
    let starks = || {
        *jon_snow = 3;
    };
    println!("{}", jon_copy);
}
