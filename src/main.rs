#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub ipc: Ipc,
    pub graphql: GraphQl,
}

#[derive(Deserialize)]
pub struct Ipc {
    pub path: Option<String>,
}

#[derive(Deserialize)]
pub struct GraphQl {
    pub port: Option<u16>,
}

fn main() {
    println!("Hello, world!");
}
