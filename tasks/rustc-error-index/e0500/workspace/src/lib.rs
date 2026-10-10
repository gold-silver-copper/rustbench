fn you_know_nothing(jon_snow: &mut i32) {
    let nights_watch = &jon_snow;
    let starks = || {
        *jon_snow = 3;
    };
    println!("{}", nights_watch);
}
