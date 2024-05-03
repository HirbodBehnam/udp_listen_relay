# UDP Listen Relay
Relay traffic between two UDP sockets.

## What

A very simple program that relays UDP traffic between two _listening_ UDP sockets. There are two sockets: Initiator and Destination. When the program starts, it sends an empty packet inside the initiator socket and then starts relaying traffic between two sockets.

## How To Run?

First you can simply build this app with `cargo build --release`. Then do run it like this:
```bash
./udp_listen_relay <INITIATOR> <DESTINATION>
```
For example:
```bash
./udp_listen_relay 127.0.0.1:12345 127.0.0.1:54321
```
