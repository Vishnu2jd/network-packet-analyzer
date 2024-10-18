use pnet::datalink;

pub fn get_default_interface() -> String {
    datalink::interfaces()
        .into_iter()
        .find(|iface| !iface.is_loopback() && iface.is_up())
        .map(|iface| iface.name)
        .unwrap_or_else(|| "eth0".to_string())
}
