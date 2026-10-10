fn inside_closure(x: &mut i32) {
}

fn outside_closure(x: &mut i32) {
}

fn foo(a: &mut i32) {
    let mut bar = || {
        inside_closure(a)
    };
    outside_closure(a);
    bar();
}
