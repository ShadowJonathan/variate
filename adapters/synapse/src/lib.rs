use std::cell::RefCell;

pub mod extract;
// pub mod insert;

use postgres::{Client, Config};
use variate_lib::{Extractor, Inserter};

pub fn make_config(
    host: &str,
    port: Option<u16>,
    user: &str,
    password: &str,
    database: &str,
) -> Config {
    let mut c = Config::new();

    c.host(host).user(user).password(password).dbname(database);

    if let Some(port) = port {
        c.port(port);
    }

    c
}

pub fn make_extractor(c: Config) -> impl Extractor {
    let mut client = c.connect(postgres::NoTls).unwrap();

    let version: i32 =
        client.query_one("SELECT version FROM schema_version", &[]).unwrap().get("version");

    assert_eq!(version, 65); // cant accept other schemas at the moment

    let puck = DatabasePuck { pg: client, version };

    extract::SynapseExtractor::new(puck)
}

pub fn make_inserter() -> Box<dyn Inserter> {
    todo!()
}

struct DatabasePuck {
    pg: Client,
    version: i32,
}
