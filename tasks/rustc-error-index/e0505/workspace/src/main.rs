struct Value {}

fn borrow(val: &Value) {}

fn eat(val: Value) {}

fn main() {
    let x = Value{};
    let _ref_to_val: &Value = &x;
    eat(x);
    borrow(_ref_to_val);
}
