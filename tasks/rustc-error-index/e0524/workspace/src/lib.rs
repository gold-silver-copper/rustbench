fn set(x: &mut isize) {
    *x += 4;
}

fn dragoooon(x: &mut isize) {
    let mut c1 = || set(x);
    let mut c2 = || set(x);

    c2();
    c1();
}
