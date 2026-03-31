pub use testcontainers;

use std::borrow::Cow;
use std::collections::HashMap;

use testcontainers::core::{ContainerPort, IntoContainerPort, Mount, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, Image};

const IMAGE_NAME: &str = "tarantool/tarantool";
const DEFAULT_IMAGE_TAG: &str = "latest";

fn image_tag() -> String {
    std::env::var("TARANTOOL_IMAGE_TAG").unwrap_or(DEFAULT_IMAGE_TAG.into())
}

pub struct TarantoolImage {
    tag: String,
    env_vars: HashMap<String, String>,
    mounts: Vec<Mount>,
    cmd: Vec<String>,
}

impl Default for TarantoolImage {
    fn default() -> Self {
        Self {
            tag: image_tag(),
            env_vars: HashMap::from([(
                "TT_MEMTX_USE_MVCC_ENGINE".into(),
                "true".into(),
            )]),
            mounts: Vec::new(),
            cmd: vec!["tarantool".into()],
        }
    }
}

impl Image for TarantoolImage {
    fn name(&self) -> &str {
        IMAGE_NAME
    }

    fn tag(&self) -> &str {
        &self.tag
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stderr("entering the event loop")]
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        &[ContainerPort::Tcp(3301)]
    }

    fn env_vars(
        &self,
    ) -> impl IntoIterator<Item = (impl Into<Cow<'_, str>>, impl Into<Cow<'_, str>>)> {
        self.env_vars.iter()
    }

    fn mounts(&self) -> impl IntoIterator<Item = &Mount> {
        self.mounts.iter()
    }

    fn cmd(&self) -> impl IntoIterator<Item = impl Into<Cow<'_, str>>> {
        self.cmd.iter()
    }
}

impl TarantoolImage {
    pub fn disable_mvcc(mut self) -> Self {
        drop(self.env_vars.remove("TT_MEMTX_USE_MVCC_ENGINE"));
        self
    }

    pub fn volume(mut self, host_path: String, container_path: String) -> Self {
        self.mounts
            .push(Mount::bind_mount(host_path, container_path));
        self
    }

    pub fn cmd(mut self, cmd: Vec<String>) -> Self {
        self.cmd = cmd;
        self
    }
}

pub struct TarantoolTestContainer {
    container: ContainerAsync<TarantoolImage>,
    port: u16,
}

impl TarantoolTestContainer {
    pub async fn new(image: TarantoolImage) -> Self {
        let container = image.start().await.expect("Failed to start Tarantool container");
        let port = container
            .get_host_port_ipv4(3301.tcp())
            .await
            .expect("Failed to get host port");
        Self { container, port }
    }

    pub async fn default_container() -> Self {
        Self::new(TarantoolImage::default()).await
    }

    pub fn connect_port(&self) -> u16 {
        self.port
    }

    pub fn container(&self) -> &ContainerAsync<TarantoolImage> {
        &self.container
    }
}
