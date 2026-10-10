async fn foo(n: usize) {
    if n > 0 {
        foo(n - 1).await;
    }
}
