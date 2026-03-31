use tarantool_rs::Connection;
use tarantool_test_container::{TarantoolImage, TarantoolTestContainer};

pub async fn new_with_test_data() -> TarantoolTestContainer {
    let image = TarantoolImage::default()
        .cmd(vec![
            "tarantool".into(),
            "/opt/tarantool/test_data.lua".into(),
        ])
        .volume(
            format!("{}/tests", env!("CARGO_MANIFEST_DIR")),
            "/opt/tarantool".into(),
        );
    TarantoolTestContainer::new(image).await
}

pub async fn create_conn(container: &TarantoolTestContainer) -> Result<Connection, tarantool_rs::errors::Error> {
    Connection::builder()
        .build(format!("127.0.0.1:{}", container.connect_port()))
        .await
}
