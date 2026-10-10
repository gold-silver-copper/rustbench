fn continue_simple() {
    'b: loop {
        continue ;
    }
}
fn continue_labeled() {
    'b: loop {
        continue 'b;
    }
}
fn continue_crossing() {
    loop {
        'b: loop {
            continue;
        }
    }
}
