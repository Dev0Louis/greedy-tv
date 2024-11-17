
use clap::Parser;

use std::any::Any;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use zeroconf::prelude::*;
use std::thread;
use zeroconf::{MdnsBrowser, ServiceDiscovery, ServiceType};
use crate::get_discoveries;

/// Example of a simple mDNS browser
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct ClientArgs {
    /// Name of the service type to browse
    #[clap(short, long, default_value = "http")]
    name: String,

    /// Protocol of the service type to browse
    #[clap(short, long, default_value = "tcp")]
    protocol: String,

    /// Sub-type of the service type to browse
    #[clap(short, long)]
    sub_type: Option<String>,
}

pub fn start() -> zeroconf::Result<()> {
    let ClientArgs {
        name,
        protocol,
        sub_type,
    } = ClientArgs::parse();

    let sub_types: Vec<&str> = match sub_type.as_ref() {
        Some(sub_type) => vec![sub_type],
        None => vec![],
    };

    let service_type =
        ServiceType::with_sub_types(&name, &protocol, sub_types).expect("invalid service type");

    let mut browser = MdnsBrowser::new(service_type);

    browser.set_service_discovered_callback(Box::new(on_service_discovered));

    let event_loop = browser.browse_services()?;

    loop {
        // calling `poll()` will keep this browser alive
        event_loop.poll(Duration::from_secs(0))?;
    }
}

fn on_service_discovered(
    result: zeroconf::Result<ServiceDiscovery>,
    _context: Option<Arc<dyn Any>>
) {
    let Ok(discovery) = &result else {
        error!("Service discovery error");
        return;
    };
    info!("Service discovered: {result:?}");
    get_discoveries().lock().unwrap().push(discovery.clone());
}