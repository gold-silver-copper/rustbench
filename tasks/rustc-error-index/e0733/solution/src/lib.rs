async fn foo(n: usize) {
    if n > 0 {
        Box::pin(foo(n - 1)).await;
    }
}
