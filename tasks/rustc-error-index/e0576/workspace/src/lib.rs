trait Hello {
    type Who;

    fn hello() -> <Self as Hello>::You;
}
