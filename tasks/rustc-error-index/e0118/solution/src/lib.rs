struct TypeWrapper<T>(T);

impl<T> TypeWrapper<T> {
    fn get_state(&self) -> String {
        "Fascinating!".to_owned()
    }
}
