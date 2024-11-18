
use clap::Parser;

use std::any::Any;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use zeroconf::prelude::*;
use zeroconf::{MdnsBrowser, ServiceDiscovery, ServiceType};
use crate::get_discoveries;

pub fn start() -> zeroconf::Result<()> {
    let mut browser = MdnsBrowser::new("_airplay._tcp".parse()?);

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