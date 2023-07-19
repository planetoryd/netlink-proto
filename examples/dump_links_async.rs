// SPDX-License-Identifier: MIT

use futures::StreamExt;
use netlink_packet_core::{NLM_F_REQUEST, NLM_F_DUMP, NetlinkHeader, NetlinkMessage};
use netlink_packet_route::{
    LinkMessage, RtnlMessage
};
use netlink_proto::{
    new_connection,
    sys::{protocols::NETLINK_ROUTE, SocketAddr},
};

#[async_std::main]
async fn main() -> Result<(), String> {
    // Create the netlink socket. Here, we won't use the channel that
    // receives unsolicited messages.
    let (conn, mut handle, _) = new_connection(NETLINK_ROUTE).map_err(|e| {
        format!("Failed to create a new netlink connection: {}", e)
    })?;

    // Spawn the `Connection` so that it starts polling the netlink
    // socket in the background.
    let _ = async_std::task::spawn(conn);

    // Create the netlink message that requests the links to be dumped
    let mut nl_hdr = NetlinkHeader::default();
    nl_hdr.flags = NLM_F_DUMP | NLM_F_REQUEST;
    let request = NetlinkMessage::new(
        nl_hdr,
        RtnlMessage::GetLink(LinkMessage::default()).into(),
    );

    // Send the request
    let mut response = handle
        .request(request, SocketAddr::new(0, 0))
        .map_err(|e| format!("Failed to send request: {}", e))?;

    // Print all the messages received in response
    loop {
        if let Some(packet) = response.next().await {
            println!("<<< {:?}", packet);
        } else {
            break;
        }
    }

    Ok(())
}
