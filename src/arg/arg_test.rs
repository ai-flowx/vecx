#[test]
fn test_argument() {
    let args = super::arg::Argument {
        ..Default::default()
    };

    assert_eq!(args.config_file.is_empty(), true);
    assert_eq!(args.listen_port.is_empty(), true);
    assert_eq!(args.repo_path.is_empty(), true);
    assert_eq!(args.version_info.is_empty(), true);
}
