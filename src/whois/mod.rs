use std::io::prelude::*;
use std::io::BufReader;
use std::net::IpAddr;
use std::net::TcpStream;

use anyhow::{anyhow, Result};
use serde::Serialize;

use crate::ip::UNKNOWN;

mod cache;

pub use cache::WhoisCache;

const WHOIS_SERVER: &str = "whois.iana.org";
const WHOIS_PORT: u16 = 43;

/// Resolve the whois server for the given IP address. This is done by connecting to
/// the IANA whois server and requesting the whois server for the given IP address:
///
/// ```
/// % IANA WHOIS server
/// % for more information on IANA, visit http://www.iana.org
/// % This query returned 1 object
///
/// refer:        whois.arin.net
///
/// inetnum:      76.0.0.0 - 76.255.255.255
/// organisation: ARIN
/// status:       ALLOCATED
///
/// whois:        whois.arin.net
///
/// changed:      2005-06
/// source:       IANA
/// ```
///
/// We're interested by the `whois:` line, which tells us the whois server to use for
/// the given IP address.
///
/// See https://www.iana.org/whois
fn get_whois_server(ip: &IpAddr) -> Result<String> {
    let mut stream = TcpStream::connect((WHOIS_SERVER, WHOIS_PORT))?;
    stream.write_all(format!("{}\n", ip).as_bytes())?;

    let reader = BufReader::new(stream);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with("whois:") {
                let parts: Vec<String> = line.splitn(2, ":").map(|x| x.to_string()).collect();

                if parts.len() == 2 {
                    return Ok(parts[1].trim().to_string());
                }
            }
        }
    }

    return Err(anyhow!("No whois server found for IP {}", ip));
}

/// Query the given whois server for the given IP address. The result is different depending
/// on the whois server, and might be recursive if the whois server returns a referral server.
fn get_whois_result(server: String, addr: &IpAddr) -> Result<Whois> {
    // The ARIN whois server requires a more specific query format, so we prepend `n` before
    // the IP address to retrieve the "network address space".
    //
    // See https://www.arin.net/resources/registry/whois/rws/api/#nicname-whois-queries
    let query = match server.as_str() {
        "whois.arin.net" => format!("n {}\n", addr),
        _ => format!("{}\n", addr),
    };

    let mut stream = TcpStream::connect((server, WHOIS_PORT))?;
    stream.write_all(query.as_bytes())?;

    let reader = BufReader::new(stream);

    let mut cidr = UNKNOWN.into();
    let mut org = UNKNOWN.into();
    let mut referral_server = None;

    for line in reader.lines() {
        if let Ok(line) = line {
            // ARIN returns `CIDR` field
            // RIPE returns `route` or `route6` fields
            if line.starts_with("CIDR:")
                || line.starts_with("route:")
                || line.starts_with("route6:")
            {
                let parts: Vec<String> = line.splitn(2, ":").map(|x| x.to_string()).collect();

                if parts.len() == 2 {
                    cidr = parts[1].trim().to_string();
                }
            }

            // ARIN returns `Organization` field
            // RIPE returns `mnt-by` field
            if line.starts_with("Organization:") || line.starts_with("netname:") {
                let parts: Vec<String> = line.splitn(2, ":").map(|x| x.to_string()).collect();

                if parts.len() == 2 {
                    org = parts[1].trim().to_string();
                }
            }

            if line.starts_with("ReferralServer:") {
                let parts: Vec<String> = line.splitn(2, ":").map(|x| x.to_string()).collect();

                if parts.len() == 2 {
                    referral_server = Some(parts[1].trim().replace("whois://", "").to_string());
                }
            }
        }
    }

    // For realocations, we need to query the referral server
    if let Some(referral_server) = referral_server {
        return get_whois_result(referral_server, addr);
    }

    Ok(Whois { cidr, org })
}

#[derive(Debug, Serialize, Clone)]
pub struct Whois {
    cidr: String,
    org: String,
}

impl Default for Whois {
    fn default() -> Self {
        Self {
            cidr: UNKNOWN.into(),
            org: UNKNOWN.into(),
        }
    }
}

impl Whois {
    pub fn new(addr: IpAddr) -> Result<Self> {
        let server = get_whois_server(&addr)?;

        get_whois_result(server, &addr)
    }
}
