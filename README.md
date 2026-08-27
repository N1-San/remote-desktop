# remote-desktop

A learning project to build a minimal remote-desktop tool from scratch in Rust —
inspired by tools like AnyDesk/TeamViewer, built incrementally to learn Rust and
systems programming (networking, screen capture, input injection).

## Status

Early development — currently validating core connectivity (TCP handshake +
PIN auth) and screen capture in isolation, on the path to a Windows-to-Windows MVP.

## Project structure

- `protocol/` — shared crate defining the wire protocol (message types, framing)
  used by both host and client
- `host/` — the machine being controlled: listens for connections, will capture
  screen + inject input
- `client/` — the machine controlling: connects to host, will render screen +
  send input

## Roadmap

- [x] Shared protocol crate with length-prefixed message framing
- [x] Basic TCP handshake between host and client with PIN check
- [x] Screen capture (single frame, JPEG-encoded) — in progress
- [ ] Wire capture into the network flow (stream frames to client)
- [ ] Render received frames on the client
- [ ] Input capture on client + injection on host
- [ ] Randomized PIN generation (currently hardcoded for testing)
- [ ] Windows ↔ Windows MVP complete
- [ ] Linux ↔ Linux support
- [ ] Cross-platform support

## Prerequisites (Linux)

Screen capture (`xcap`) depends on system libraries not included with Rust.
Install these before building:

**Fedora:**
```bash
sudo dnf install dbus-devel pkgconf-pkg-config libxcb-devel
```

**Debian/Ubuntu:**
```bash
sudo apt install libdbus-1-dev pkg-config libxcb1-dev
```

(Windows has no equivalent requirement — this only applies to building/running
`host` on Linux.)

## Running locally

Requires Rust (`rustup`) installed.

```bash
cargo build

# Terminal 1
cargo run -p host

# Terminal 2
cargo run -p client
```

## Testing across two machines

See `client/src/main.rs` — the host IP is currently entered interactively
when running the client binary. Run `host` on one machine, `client` on
another, on the same network.

## License

Personal learning project — no license applied yet.