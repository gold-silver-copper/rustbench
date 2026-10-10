use task::ServerConfig;

#[test]
fn default_values() {
    let c = ServerConfig::default();
    assert_eq!(c.host, "127.0.0.1");
    assert_eq!(c.port, 8080);
    assert_eq!(c.max_connections, 64);
    assert!(!c.verbose);
    assert_eq!(c.address(), "127.0.0.1:8080");
    let custom = ServerConfig { port: 9000, ..Default::default() };
    assert_eq!(custom.address(), "127.0.0.1:9000");
}

#[test]
fn parser_unchanged() {
    assert_eq!(task::parse::parse("").unwrap(), ServerConfig::default());
    let c = task::parse::parse("host = 0.0.0.0\nmax_connections = 10 # small\n").unwrap();
    assert_eq!(c.address(), "0.0.0.0:8080");
    assert_eq!(c.max_connections, 10);
    assert!(task::parse::parse("port = lots").is_err());
}
