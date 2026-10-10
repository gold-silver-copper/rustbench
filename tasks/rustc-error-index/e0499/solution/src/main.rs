fn main() {
    let mut i = 0;
    let mut x = &mut i;

    let mut i = 0;
    let a = &i;
    let b = &i;
    let c = &i;
    b;
    a;
}
