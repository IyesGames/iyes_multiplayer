# IyesGames Multiplayer Infrastructure

This project is where all our general (game-agnostic) multiplayer-related code
is developed. The goal is to develop a secure and practical common protocol for
game session management and player authentication.

Warning: early WIP. This software is not considered ready for use yet.

## Auth Server

The Auth Server is the central server that game clients connect to, when they
are not in a game session. It is responsible for:

- Keeping track of player accounts and authenticating them (optional)
- Putting players into game sessions together (lobbies, matchmaking?)
- Picking a Host Server for them to play on
- Hand-off to the Host Server

### Security Model

The Auth Server does not participate in gameplay, and therefore is not
performance-critical. It can be hosted behind a proxy or DDOS-protection service
such as Cloudflare.

If the Auth Server is disrupted, no new game sessions can begin, but players who
are already playing the game will be unaffected (as they are connected to a Host
Server, and the Auth Server is not involved).

## Host Server

Host Servers are where gameplay actually happens. They get session information
from the Auth Server, wait for the players to connect, and then enter the
game-specific code.

There can be multiple Host Servers (such as for different geographic regions),
and the Auth Server is responsible for choosing which one players play on.

### Security Model

A Host Server should only accept network packets from whitelisted clients, based
on the session information provided by the Auth Server. All other traffic is to
be dropped, to prevent DDOS. Host Servers should be connected directly to the
Internet, ideally hosted at a location optimized for low latency.

## Clients

The game client connects to the Auth Server first. Typically this happens in the
game's main menu or multiplayer menu. The client communicates with the Auth
Server to find other players to play with and request a game session with
whatever settings it wants.

When everything is ready, Hand-Off is performed. The client connects to the Host
Server. The initial (readying) stage of a game session is performed. When all
players are connected and ready, gameplay begins. From that point onwards, a
game-specific protocol is used.

## Hand-Off

When the Auth Server wants to create a new game session, Hand-Off is performed.

The Auth Server sends session metadata to the Host Server and Clients.

The Host gets the IP address and Session certificate of each client. It can then
whitelist these IPs, expect QUIC connections, and perform client certificate
verification.

The Clients get the IP of the Host server they will play on, and their new
freshly-minted Session certificate + private key, which they should use when
connecting to the Host. They can then disconnect from the Auth server, connect
to the Host, and ready up to begin playing.

## Protocols

All protocols are based on QUIC as the transport. Datastructures are serialized
using MsgPack. There is an elaborate TLS security model described below.

All of the protocols are extensible, allowing game-specific extras to be added.

### Security Model

TLS certificates are used to prevent abuse. Note: this is *not* a DRM scheme.
The goal is to allow server operators to easily avoid (most) unwanted traffic,
and connections to be secure and trusted. It does not stop malicious clients.
If functionality like "account verification" is desired, that can be implemented
as part of the Auth Server protocol.

The Master certificate is the top-level CA certificate of the server operator
(your organization / game studio / publisher).

The Auth and Host servers have certificates signed by the Master certificate,
which allows clients to verify them upon connecting.

ClientAuth certificates are secondary CA certificates signed by the Master,
which are used by the Auth server to verify incoming client connections. Clients
must present a certificate signed by an active ClientAuth. This allows the
server operator to easily drop support for outdated/unsupported clients by using
a new ClientAuth to sign new game updates and dropping the old one.

Clients use their Client certificate (signed with ClientAuth, distributed with
the game install) to connect to the Auth Server.

The SessionAuth certificate is the CA for Session certificates. It is signed by
the Master and used for client verification by the Host Server.

On Hand-Off, the Auth server generates fresh new Session certificates for each
client, to be used only for that game session. It sends those certificates to
the Host server, along with the session metadata. Each client gets its private
key, to be used to connect to the Host server.

## Implementation

`iyes_multiplayer_{authsrv,hostsrv,client}` are Rust library crates that
implement the architecture in a game-agnostic way. They are based on
[Tokio](https://github.com/tokio-rs/tokio),
[Quinn](https://github.com/quinn-rs/quinn), and
[Rustls](https://github.com/rustls/rustls).

They are designed to be easily extended with game-specific code, to create the
servers and clients for different game projects. Everything is pluggable.
Game-specific extra data can be added to all of the protocols, player
authentication can be customized, and the host server can do anything when
gameplay actually begins.

The respective `examples` folders contain examples that show how to make an
actual Auth server / Host server / Client binary. There are examples for a
simple CLI game, as well as for [Bevy](https://github.com/bevyengine/bevy)
integration.

While the `iyes_multiplayer` architecture was designed with dedicated servers in
mind, the Auth/Host server roles can easily be performed by a game client, for
LAN/self-hosted scenarios.

`iyesmpcli` is a CLI tool for developers and administrators, to help with
deploying and managing game servers and clients based on `iyes_multiplayer`.
Its most common use is to generate all the TLS certificates.
