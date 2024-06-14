use anyhow::Result;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use std::net::IpAddr;
use uuid::Uuid;

use super::Header;

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub id: Uuid,
    #[serde(rename = "routerId")]
    pub router_id: Uuid,
    #[serde(deserialize_with = "deserialize_ipaddr")]
    pub address: IpAddr,
    pub port: u16,
    pub name: String,
    pub description: String,
    pub headers: Vec<Header>,
}

impl Server {
    #[allow(dead_code)]
    pub fn new(router_id: Uuid, address: &str, port: u16) -> Result<Self> {
        let address = address.parse::<IpAddr>()?;

        Ok(Server {
            id: Uuid::new_v4(),
            router_id,
            address,
            port,
            name: String::default(),
            description: String::default(),
            headers: vec![],
        })
    }

    #[allow(dead_code)]
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    #[allow(dead_code)]
    pub fn set_description(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    #[allow(dead_code)]
    pub fn add_header(&mut self, header: Header) -> &mut Self {
        self.headers.push(header);
        self
    }
}

// Custom deserialization function for IpAddr
fn deserialize_ipaddr<'de, D>(deserializer: D) -> Result<IpAddr, D::Error>
where
    D: Deserializer<'de>,
{
    struct IpAddrVisitor;

    impl<'de> Visitor<'de> for IpAddrVisitor {
        type Value = IpAddr;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid IPv4 or IPv6 address")
        }

        fn visit_str<E>(self, value: &str) -> Result<IpAddr, E>
        where
            E: de::Error,
        {
            value.parse().map_err(de::Error::custom)
        }
    }

    deserializer.deserialize_str(IpAddrVisitor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::uuid;

    #[test]
    fn can_deserialize_server() {
        let json = r###"
        {
            "id": "a3c6bf8d-57ab-4693-94dd-4d3a47dd1403",
            "address": "127.0.0.1",
            "port": 8080,
            "name": "Unit Test",
            "headers": [
                {
                    "id": "5ea28783-d770-4754-98e9-749aab005511",
                    "key": "Content-Type",
                    "value": "application/html",
                    "active": true
                }
            ],
            "routerId": "d49427b0-8d2f-47cc-a5b4-c3754c53583b",
            "description": "Just an example of a server in JSON."
        }
        "###;

        let server: Server = serde_json::from_str(json).expect("Unable to parse JSON.");
        assert_eq!(server.id, uuid!("a3c6bf8d-57ab-4693-94dd-4d3a47dd1403"));
        assert_eq!(server.address.to_string(), "127.0.0.1");
        assert_eq!(server.port, 8080);
        assert_eq!(server.name, "Unit Test");
        assert_eq!(server.headers.len(), 1);
        assert_eq!(
            server.headers.first().unwrap().id,
            uuid!("5ea28783-d770-4754-98e9-749aab005511")
        );
        assert_eq!(
            server.router_id,
            uuid!("d49427b0-8d2f-47cc-a5b4-c3754c53583b")
        );
        assert_eq!(server.description, "Just an example of a server in JSON.");
    }

    #[test]
    fn cannot_use_invalid_ip_address() {
        let server = Server::new(Uuid::new_v4(), "invalid.ip.addr", 0);
        assert!(server.is_err());

        let json = r###"
        {
            "id": "a3c6bf8d-57ab-4693-94dd-4d3a47dd1403",
            "address": "invalid.ip.addr",
            "port": 8080,
            "name": "Unit Test",
            "headers": [
                {
                    "id": "5ea28783-d770-4754-98e9-749aab005511",
                    "key": "Content-Type",
                    "value": "application/html",
                    "active": true
                }
            ],
            "routerId": "d49427b0-8d2f-47cc-a5b4-c3754c53583b",
            "description": "Just an example of a server in JSON."
        }
        "###;

        let server = serde_json::from_str::<Server>(json);
        assert!(server.is_err());
    }
}
