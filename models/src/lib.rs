use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub host_name: String,
    mac_addr: String,
    ip: String,
    address_source: AddressSource,
    connect_type: ConnectType,
    comnum: u32,
    app_enable: BooleanFlag,
    action: String,
    is_extender: BooleanFlag,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AddressSource {
    #[serde(rename = "DHCP-IP")]
    DhcpIp,
    #[serde(rename = "Self-assigned")]
    SelfAssigned,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ConnectType {
    #[serde(rename = "WiFi 5G")]
    Wifi5G,
    #[serde(rename = "WiFi 2.4G")]
    Wifi24G,
    #[serde(rename = "Ethernet")]
    Ethernet,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum BooleanFlag {
    #[serde(rename = "TRUE")]
    True,
    #[serde(rename = "FALSE")]
    False,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostResponse {
    err_code: String,
    err_msg: String,
    #[serde(rename = "HostNumberOfEntries")]
    host_number_of_entries: String,
    timestamp: String,
    #[serde(rename = "Hosts_List")]
    pub hosts_list: Vec<Host>,
}

