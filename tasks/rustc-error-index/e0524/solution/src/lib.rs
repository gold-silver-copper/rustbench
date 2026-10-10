fn set(x: &mut isize) {
    *x += 4;
}

fn dragoooon(x: &mut isize) {
    {
        let mut c1 = || set(&mut *x);
        c1();
    }
    let mut c2 = || set(&mut *x);
    c2();
}
