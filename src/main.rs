extern crate docopt;
extern crate multicast_dns;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;

use multicast_dns::discovery::discovery_manager::*;
use multicast_dns::host::HostManager;

const USAGE: &str = "
Usage: multicast-dns-utils [-t <type>] [-n <hostname>] [-a <alias>]

Options:
    -t, --type <type>       Look for service of the specified type (e.g. _device-info._tcp).
    -n, --name <hostname>   Set custom host name.
    -a, --alias <alias>     Set custom host name alias.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_type: Option<String>,
    flag_name: Option<String>,
    flag_alias: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let empty_string = "".to_owned();

    if args.flag_type.is_some() {
        let discovery_manager = DiscoveryManager::new();

        let on_service_resolved = |service: ServiceInfo| {
            println!(
                "=   {}   {:?}   {}   {}   {}",
                service.interface,
                service.protocol,
                service.name.as_ref().unwrap_or(&empty_string),
                service.type_name.as_ref().unwrap_or(&empty_string),
                service.domain.as_ref().unwrap_or(&empty_string)
            );
            println!(
                "    hostname = [{}]",
                service.host_name.as_ref().unwrap_or(&empty_string)
            );
            println!(
                "    address = [{}]",
                service.address.as_ref().unwrap_or(&empty_string)
            );
            println!("    port = [{}]", service.port);
            println!(
                "    txt = [{}]",
                service.txt.as_ref().unwrap_or(&empty_string)
            );
        };

        let on_service_discovered = |service: ServiceInfo| {
            println!(
                "+   {}   {:?}   {}   {}   {}",
                service.interface,
                service.protocol,
                service.name.as_ref().unwrap_or(&empty_string),
                service.type_name.as_ref().unwrap_or(&empty_string),
                service.domain.as_ref().unwrap_or(&empty_string)
            );

            discovery_manager.resolve_service(
                service,
                ResolveListeners {
                    on_service_resolved: Some(&on_service_resolved),
                },
            );
        };

        let on_all_services_discovered = || {
            discovery_manager.stop_service_discovery();
        };

        let discovery_listeners = DiscoveryListeners {
            on_service_discovered: Some(&on_service_discovered),
            on_all_discovered: Some(&on_all_services_discovered),
        };

        match discovery_manager.discover_services(&args.flag_type.unwrap(), discovery_listeners) {
            Ok(_) => println!("All services has been discovered. Stopping discovery..."),
            Err(err) => println!("\x1B[31m{}\x1B[0m", err),
        }
    }

    if args.flag_name.is_some() {
        let host_manager = HostManager::new();
        let new_host_name = args.flag_name.unwrap();

        match host_manager.get_name().and_then(|current_name| {
            println!(
                "Hostname update ({} -> {}) is requested",
                current_name, &new_host_name
            );
            host_manager.set_name(&new_host_name)
        }) {
            Ok(new_name) => println!("New Host name: {:?}", new_name),
            Err(err) => println!("\x1B[31m{}\x1B[0m", err),
        }
    }

    if args.flag_alias.is_some() {
        let host_manager = HostManager::new();
        let new_alias = args.flag_alias.unwrap();

        println!("New alias ({}) is requested", &new_alias);

        match host_manager.add_name_alias(&new_alias) {
            Ok(_) => {
                println!(
                    "New alias \"{}\" is active until program is terminated.",
                    new_alias
                );

                loop {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }
            Err(err) => println!("\x1B[31m{}\x1B[0m", err),
        }
    }
}
