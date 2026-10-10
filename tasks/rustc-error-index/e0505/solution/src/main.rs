struct Value {}

fn borrow(val: &Value) {}

fn eat(val: &Value) {}

fn main() {
    let x = Value{};

    let ref_to_val: &Value = &x;
    eat(&x);
    borrow(ref_to_val);
}
