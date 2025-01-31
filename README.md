# GnosisVPN Client

The client establishes a VPN connection to a remote endpoint.
It acts as a monitoring/management layer of hoprd session handling and WireGuard setup (Linux only).

## Onboarding

Follow [ONBOARDING](./ONBOARDING.md) guide to use GnosisVPN.

## General usage

The client is meant to run as a service binary with privileged access.
The default configuration file is located in `/etc/gnosisvpn/config.toml`.
However you can start the client with

`GNOSISVPN_CONFIG_PATH=<config_file> GNOSISVPN_SOCKET_PATH=<socket_path> ./gnosis_vpn-<system-arch>`

from userspace (`socket_path` being some accessible file location, e.g. ./gnosisvpn.sock).

A minimal configuration file is [config.toml](./config.toml).
Use [documented-config.toml](./documented-config.toml) as a full reference.

### Env vars

`GNOSISVPN_CONFIG_PATH` - path to the configuration file
`GNOSISVPN_SOCKET_PATH` - path to the control socket

## Deployment

Show potential deployment targets:

`nix flake show`

Build for a target, e.g. `x86_64-linux`:

`nix build .#gnosisvpn-x86_64-linux`

The resulting binaries are in `result/bin/`:

```
$ ls -l result*/bin/
result/bin/:
total 4752
-r-xr-xr-x 1 root root 4863368 Jan  1  1970 gnosis_vpn
-r-xr-xr-x 1 root root 1740048 Jan  1  1970 gnosis_vpn-ctl
```
