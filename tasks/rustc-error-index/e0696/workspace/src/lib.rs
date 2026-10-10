fn continue_simple() {
    'b: {
        continue;
    }
}
fn continue_labeled() {
    'b: {
        continue 'b;
    }
}
fn continue_crossing() {
    loop {
        'b: {
            continue;
        }
    }
}
