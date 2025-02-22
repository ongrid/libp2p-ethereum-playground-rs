use futures::prelude::*;
use libp2p::{
    core::upgrade,
    TransportExt, // Provides the upgrade() method.
    identity,
    multiaddr::Multiaddr,
    PeerId, Transport,
};
use libp2p_tcp as tcp;
use libp2p_tcp::tokio::Transport as TcpTransport;
use libp2p_swarm::{Swarm, SwarmEvent};
use libp2p_noise::{Config as NoiseConfig};
use libp2p_yamux::Config as YamuxConfig;
/*
use libp2p_gossipsub::{
    Gossipsub, GossipsubConfigBuilder, GossipsubEvent, IdentTopic as Topic,
    MessageAuthenticity, IdentityTransform, subscription_filter::AllowAllSubscriptionFilter,
};
*/
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Step 1. Generate a local identity.
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    // Step 2. Build a TCP transport using libp2p_tcp's tokio integration.
    let tcp_config = tcp::Config::default();
    let tcp_transport = TcpTransport::new(tcp_config);
    
    // Step 3. Upgrade the TCP transport:
    //   - Use protocol Version V1.
    //   - Authenticate using Noise (XX handshake).
    //   - Multiplex using Yamux.
    
    //let noise_keys = NoiseKeypair::into_authentic(&local_key)
    //    .expect("Failed to create noise keys");

    let transport = tcp_transport
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::new(&local_key).into_authenticated())
        .multiplex(YamuxConfig::default())
        .boxed();

    // Step 4. Build a Gossipsub behavior.
    /*
    let gossipsub_config = GossipsubConfigBuilder::default().build()?;
    let mut gossipsub = Gossipsub::new(
        MessageAuthenticity::Signed(local_key.clone()),
        gossipsub_config,
    )?;
    // Create and subscribe to a topic.
    let topic = Topic::new("lighthouse_ping");
    gossipsub.subscribe(&topic).expect("Subscription failed");

    // Step 5. Build the Swarm.
    let mut swarm = Swarm::with_async_std_executor(transport, gossipsub, local_peer_id.clone());
    */

    // Step 6. Listen on an ephemeral port.
    let listen_addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse()?;
    // swarm.listen_on(listen_addr)?;
    println!("Listening...");

    // Wait until a listen address is reported.
    //while let Some(event) = swarm.select_next_some().await {
    //    if let SwarmEvent::NewListenAddr { address, .. } = event {
    //        println!("Now listening on {:?}", address);
    //        break;
    //    }
    //}

    // Step 7. Dial a remote peer.
    let remote_addr: Multiaddr = "/ip4/176.199.96.212/tcp/9001/p2p/16Uiu2HAm4R7SBokmsSPqZAsAzSkUCbWsxLCnLGHSMxVqAoSzBU8h"
        .parse()?;
    //swarm.dial(remote_addr)?;
    //println!("Dialed remote peer");

    // Step 8. Publish a "Ping" message on our topic.
    let ping = "Ping".as_bytes().to_vec();
    //swarm.behaviour_mut().publish(topic.clone(), ping)?;
    println!("Published Ping");

    // Step 9. Wait for a response.
    //while let Some(event) = swarm.select_next_some().await {
    //    match event {
    //        SwarmEvent::Behaviour(GossipsubEvent::Message { message, .. }) => {
    //            println!("Received: {}", String::from_utf8_lossy(&message.data));
    //            break;
    //        }
    //        _ => {}
    //    }
    //}

    println!("Client shutting down.");
    Ok(())
}