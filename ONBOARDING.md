# Onboarding

Setting up the GnosisVPN PoC can be somewhat complex, as it was designed as a technical showcase rather than a full-fledged product.
If you are not comfortable editing configuration files or using your terminal, please wait for the next version, which will offer a more streamlined user experience.

The Gnosis VPN proof of concept is a work in progress which should not be used in situations which require full anonymity.
To manage load and aid with testing, an allow list of sites is currently enforced: the full allow list can be viewed [here](https://gnosisvpn.com/servers).
For testing and debugging, exit nodes currently gather logs.
These logs cannot reveal user requests, server responses, IPs or any other identifying information to Gnosis or any other party.
Logs are deleted after thirty days.

Outline:

- **Manually prepare** and configure a WireGuard interface on top of your GnosisVPN connection.
- **Configure your hopd node** to allow a GnosisVPN connection.
- **Download the binary file** and run it with several env var parameters.
- **Configure GnosisVPN client** using information from three separate sources:
  1. Your hoprd node credentials
  2. Your assigned device IP
  3. Your chosen exit location
- **Configure Firefox proxy settings** to use the GnosisVPN connection.

Please select your operating system to begin:

- [Instructions for MacOS](#instructions-for-macos)
- [Instructions for Linux](#instructions-for-linux)

---

## Instructions for MacOS

### 1. Generate Wireguard public key [MacOS]

1. Download the [WireGuard app](https://apps.apple.com/us/app/wireguard/id1451685025) from the Mac App Store.
2. Launch WireGuard, create an **Empty tunnel**, name it, and save. Copy the public key of the newly created tunnel.

### 2. Prepare secure input to receive assigned device IP [MacOS]

Create a secure input location where you will receive your assigned device IP.

1. Go to [rlim.com](https://rlim.com).
2. Locate the "Custom URL" input field and enter your desired text (e.g., `toms-feedback-gvpn`). Click on "Post".
   Leave the "Custom Edit Code" field empty. An edit code will be generated automatically.
3. Save the generated URL from the browser's address bar (e.g., `https://rlim.com/toms-feedback-gvpn`).
4. Note the edit code at the top for the next step.

### 3. Provide necessary data to be eligible for GnosisVPN PoC demo [MacOS]

Provide your public key, the **rlim.com** URL, and the edit code in our [onboarding form](https://cryptpad.fr/form/#/2/form/view/bigkDtjj+9G3S4DWCHPTOjfL70MJXdEWTDjkZRrUH9Y/).
If you have trouble opening cryptpad, please try to open it in incognito mode.

### 4. Wait until you get your assigned device IP [MacOS]

After someone picked up your public key and added it to our WireGuard servers you will find your assigned device IP at your **rlim.com** document.
If you provided a communication channel (email/telegram) in the onboarding form, you will be notified.
Otherwise, you will just have to check your **rlim.com** document yourself after a reasonable amount of time.

### 5. Configure your hoprd node to allow GnosisVPN connections [MacOS]

GnosisVPN will create UDP connection to your hoprd node on a specified port (e.g.: `1422`).

Treat this as an additional port for hoprd that needs the same treatment as the peer-to-peer port and API port.
If you set up any firewall rules or port forwarding for those ports you will need to do the same for GnosisVPN port.

Additionally you need to configure your hoprd node to allow GnosisVPN connections.
The usual way of running horpd is in a docker container.
This means you need to configure docker to forward that port.

Depending on your setup this can be done in different ways.

#### Hoprd for Docker [MacOS]

Update the run command to inlude the port forwarding: `docker run ... -p 1422:1422/udp ...`.

#### Hoprd for Docker Compose [MacOS]

Locate `docker-compose.yaml` update update the `ports:` section of `hoprd:`:

```yaml
services:
  hoprd:
    ...
    ports:
      ...
      - "1422:1422/udp"
```

#### Hoprd for Dappnode [MacOS]

1. Connect to your Dappnode.
2. Navigate to the **HOPR package**.
3. Go to the **Network** tab and locate the **Public Port Mapping** section.
4. Add a new port entry by clicking on **New port +**.
5. Configure the following settings:
   - **HOST PORT**: `1422`
   - **PACKAGE PORT NUMBER**: `1422`
   - **PROTOCOL**: Select **UDP**.
6. Click **Update Port Mappings** to save your changes.

### 6. Download the latest binary file [MacOS]

Download the latest GnosisVPN binary file for your system by visiting the [GitHub releases](https://github.com/hoprnet/gnosis-vpn-client-system-service/releases) page.
Choose the binary file that matches your system:

| System                | Binary file                 |
| --------------------- | --------------------------- |
| macOS with ARM chip   | `gnosis_vpn-aarch64-darwin` |
| macOS with Intel chip | `gnosis_vpn-x86_64-darwin`  |

Ignore the `*-ctl-*` sibling files.
We do not need them for now.

### 7. Configure GnosisVPN client - hoprd node [MacOS]

1. Download [config](./config.toml) and place it next to the downloaded binary file.
2. Open `config.toml` in edit mode and locate `[hoprd_node]` section to adjust these values:

```toml
[hoprd_node]
endpoint = "http://123.456.7.89:3002"
api_token = "<hoprd node API token>"

internal_connection_port = 1422
```

- `endpoint` is the URL (including port) pointing to the API access of your node (e.g., `http://123.456.7.89:3002`).
- `api_token` is the API access token of your node.
- `internal_connection_port` is the static UDP port of your hoprd node on which GnosisVPN will establish a connection.

If you like a more extensively documented configuration file try using [documented config](./documented-config.toml).

### 8. Configure GnosisVPN client - exit location [MacOS]

Visit [GnosisVPN servers](https://gnosisvpn.com/servers) and choose an exit location.
Copy the settings into your `config.toml`:

```toml
[connection]
destination = "<exit node peer id>"

[connection.target]
host = "<exit node connection target host>"

[connection.path]
intermediates = [ `<relay node peer id>` ]
```

Save and close the configuration file.

### 9. Ensure Pathfinding to GnosisVPN Exit Nodes [MacOS]

**Caution:** If you have **channel auto-funding** enabled, you might drain your funds quickly.
To verify this, connect to your node via **Admin UI** and navigate to the **Configuration** page.
Look at the **Strategies** section and ensure that `!AutoFunding` is **not** enabled.

**Important Note:** Currently GnosisVPN can only establish connections through high-profile relay nodes maintained by the community.
To use GnosisVPN, you must have an open payment channel from your entry node to the relayer node associated with your chosen exit node.
Relay node address can be found on the [GnosisVPN servers](https://gnosisvpn.com/servers) page.

#### Steps to Open a Payment Channel [MacOS]

1. Connect to your node via **Admin UI**.
2. Navigate to the **PEERS** page.
3. Search for the peer you’ve chosen as a relayer node from [GnosisVPN servers](https://gnosisvpn.com/servers).
4. Click on **OPEN outgoing channel**.
5. Enter funding amount (recommended: **10 wxHOPR**) and click **Open Channel**.
6. Once the channel is successfully opened, it will appear under the **CHANNELS: OUT** page.

### 10. Launch the GnosisVPN binary file [MacOS]

1. Return to your downloaded binary file and make it executable by executing the following command in your terminal:

```bash
chmod +x ./gnosis_vpn-aarch64-darwin
# depending on your system, alternatively: chmod +x ./gnosis_vpn-x86_64-darwin
```

2. Provide the path to your configuration file and a socket path to launch the GnosisVPN binary file.
   The socket path is only used for communication with the GnosisVPN client which is out of scope for this guide.
   If you do not want to provide a socket path, you can also start the binary file with privileged access and it will use `/var/run/gnosis_vpn.sock` as it's communication socket.

```bash
# <system> matches the one you chose earlier
GNOSISVPN_CONFIG_PATH=./config.toml GNOSISVPN_SOCKET_PATH=./gnosis_vpn.sock ./gnosis_vpn-aarch64-darwin
# depending on your system, alternatively: GNOSISVPN_CONFIG_PATH=./config.toml GNOSISVPN_SOCKET_PATH=./gnosis_vpn.sock ./gnosis_vpn-x86_64-darwin

# or with privileged access
sudo GNOSISVPN_CONFIG_PATH=./config.toml ./gnosis_vpn-aarch64-darwin`
# depending on your system, alternatively: sudo GNOSISVPN_CONFIG_PATH=./config.toml ./gnosis_vpn-x86_64-darwin`
```

3. Because of macOS security settings, you will see a message that says binary file “cannot be opened because the developer cannot be verified”.
   Click "Cancel" or "Done", then open System Settings → Privacy & Security, scroll down to Security, and find the blocked binary file. Click "Allow Anyway".

4. In your terminal, run the command to start the binary file again. MacOS will prompt you one more time to confirm if you want to open it. Click "Open" or "Open anyway".

If you see immediate errors on startup it is most likely due to errors in your configuration settings.
The binary file should tell you which setting parameter might be wrong.

### 11. Update the newly created WireGuard tunnel and launch WireGuard [MacOS]

In the WireGuard app, edit the tunnel you created.
Replace placeholders `<...>` with the actual values as documented.

```conf
[Interface]
PrivateKey = <Generated automatic by WireGuard app>
ListenPort = 51820
Address = <device IP> # received via drop location, e.g.: 20.0.0.5/32

[Peer]
PublicKey = <wg server pub key> # listed on https://gnosisvpn.com/servers
Endpoint = <hoprd node IP:1422> # port needs to match your `internal_connection_port` configuration
AllowedIPs = 20.0.0.0/24
PersistentKeepalive = 30
```

Now you can activate this interface to establish a connection.

### 12. Use GnosisVPN connection to browse the internet [MacOS]

For now we only allow SOCKS v5 proxy connections tunneled through GnosisVPN.
The easiest way to do this is to change the Firefox proxy settings.

1. Open Network Connection Settings by navigating into Settings → General → Network Settings or search "proxy" in the settings search bar and click on the "Settings" button.
2. Choose manual proxy configuration and enter:
   - SOCKS Host: `20.0.0.1`
   - Port: `3128`
   - Socks v5
3. Clik "OK" to save the settings.

Start browsing [these select sites](https://gnosisvpn.com/servers#whitelisted) through GnosisVPN.

---

## Instructions for Linux

### 1. Generate WireGuard keypair [Linux]

Follow guidelines on official [WireGuard documentation](https://www.wireguard.com/quickstart/#key-generation).
Usually:

```bash
wg genkey | tee privatekey | wg pubkey > publickey
```

### 2. Prepare secure input to receive assigned device IP [Linux]

Create a secure input location where you will receive your assigned device IP.

1. Go to [rlim.com](https://rlim.com).
2. Locate the "Custom URL" input field and enter your desired text (e.g., `toms-feedback-gvpn`). Click on "Post".
   Leave the "Custom Edit Code" field empty. An edit code will be generated automatically.
3. Save the generated URL from the browser's address bar (e.g., `https://rlim.com/toms-feedback-gvpn`).
4. Note the edit code at the top for the next step.

### 3. Provide necessary data to be eligible for GnosisVPN PoC demo [Linux]

1. Preview public key:

```bash
cat publickey | xclip -r -sel clip
```

2. Provide your public key, the **rlim.com** URL, and the edit code in our [onboarding form](https://cryptpad.fr/form/#/2/form/view/bigkDtjj+9G3S4DWCHPTOjfL70MJXdEWTDjkZRrUH9Y/).
   If you have trouble opening cryptpad, please try to open it in incognito mode.

### 4. Wait until you get your assigned device IP [Linux]

After someone picked up your public key and added it to our WireGuard servers you will find your assigned device IP at your **rlim.com** document.
If you provided a communication channel (email/telegram) in the onboarding form, you will be notified.
Otherwise, you will just have to check your **rlim.com** document yourself after a reasonable amount of time.

### 5. Configure your hoprd node to allow GnosisVPN connections [Linux]

GnosisVPN will create UDP connection to your hoprd node on a specified port (e.g.: `1422`).

Treat this as an additional port for hoprd that needs the same treatment as the peer-to-peer port and API port.
If you set up any firewall rules or port forwarding for those ports you will need to do the same for GnosisVPN port.

Additionally you need to configure your hoprd node to allow GnosisVPN connections.
The usual way of running horpd is in a docker container.
This means you need to configure docker to forward that port.

Depending on your setup this can be done in different ways.

#### Hoprd for Docker [Linux]

Update the run command to inlude the port forwarding: `docker run ... -p 1422:1422/udp ...`.

#### Hoprd for Docker Compose [Linux]

Locate `docker-compose.yaml` update update the `ports:` section of `hoprd:`:

```yaml
services:
  hoprd:
    ...
    ports:
      ...
      - "1422:1422/udp"
```

#### Hoprd for Dappnode [Linux]

1. Connect to your Dappnode.
2. Navigate to the **HOPR package**.
3. Go to the **Network** tab and locate the **Public Port Mapping** section.
4. Add a new port entry by clicking on **New port +**.
5. Configure the following settings:
   - **HOST PORT**: `1422`
   - **PACKAGE PORT NUMBER**: `1422`
   - **PROTOCOL**: Select **UDP**.
6. Click **Update Port Mappings** to save your changes.

### 6. Download the latest binary file [Linux]

Download the latest GnosisVPN binary file for your system by visiting the [GitHub releases](https://github.com/hoprnet/gnosis-vpn-client-system-service/releases) page.
Choose the binary file that matches your system:

| system                    | binary file                |
| ------------------------- | -------------------------- |
| linux with x86 chip       | `gnosis_vpn-x86_64-linux`  |
| linux with newer ARM chip | `gnosis_vpn-aarch64-linux` |
| linux with older ARM chip | `gnosis_vpn-armv7l-linux`  |

Ignore the `*-ctl-*` sibling files.
We do not need them for now.

### 7. Configure GnosisVPN client - hoprd node [Linux]

1. Download [config](./config.toml) and place it next to the downloaded binary file.
2. Open `config.toml` in edit mode and locate `[hoprd_node]` section to adjust these values:

```toml
[hoprd_node]
endpoint = "http://123.456.7.89:3002"
api_token = "<hoprd node API token>"

internal_connection_port = 1422
```

- `endpoint` is the URL (including port) pointing to the API access of your node (e.g., `http://123.456.7.89:3002`).
- `api_token` is the API access token of your node.
- `internal_connection_port` is the static UDP port of your hoprd node on which GnosisVPN will establish a connection.

If you like a more extensively documented configuration file try using [documented config](./documented-config.toml).

### 8. Configure GnosisVPN client - exit location [Linux]

Visit [GnosisVPN servers](https://gnosisvpn.com/servers) and choose an exit location.
Copy the settings into your `config.toml`:

```toml
[connection]
destination = "<exit node peer id>"

[connection.target]
host = "<exit node connection target host>"

[connection.path]
intermediates = [ `<relay node peer id>` ]
```

Save and close the configuration file.

### 9. Ensure Pathfinding to GnosisVPN Exit Nodes [Linux]

**Caution:** If you have **channel auto-funding** enabled, you might drain your funds quickly.
To verify this, connect to your node via **Admin UI** and navigate to the **Configuration** page.
Look at the **Strategies** section and ensure that `!AutoFunding` is **not** enabled.

**Important Note:** Currently GnosisVPN can only establish connections through high-profile relay nodes maintained by the community.
To use GnosisVPN, you must have an open payment channel from your entry node to the relayer node associated with your chosen exit node.
Relay node address can be found on the [GnosisVPN servers](https://gnosisvpn.com/servers) page.

#### Steps to Open a Payment Channel [Linux]

1. Connect to your node via **Admin UI**.
2. Navigate to the **PEERS** page.
3. Search for the peer you’ve chosen as a relayer node from [GnosisVPN servers](https://gnosisvpn.com/servers).
4. Click on **OPEN outgoing channel**.
5. Enter funding amount (recommended: **10 wxHOPR**) and click **Open Channel**.
6. Once the channel is successfully opened, it will appear under the **CHANNELS: OUT** page.

### 10. Ready to start the GnosisVPN binary file [Linux]

Replace `<gnosis_vpn-binary>` with the binary file you downloaded earlier, see [step 6](#6-download-the-latest-binary-file-linux).

1. Return to your downloaded binary file and make it executable:

```bash
chmod +x <gnosis_vpn-binary>
```

2. Provide the path to your configuration file and a socket path to launch the GnosisVPN binary file.
   The socket path is only used for communication with the GnosisVPN client which is out of scope for this guide.
   If you do not want to provide a socket path, you can also start the binary file with privileged access and it will use `/var/run/gnosis_vpn.sock` as it's communication socket.

```bash
# without privileged access
GNOSISVPN_CONFIG_PATH=./config.toml GNOSISVPN_SOCKET_PATH=./gnosis_vpn.sock <gnosis_vpn-binary>
# with privileged access
sudo GNOSISVPN_CONFIG_PATH=./config.toml <gnosis_vpn-binary>
```

If you see immediate errors on startup it is most likely due to errors in your configuration settings.
The binary file should tell you which setting parameter might be wrong.

### 11. Create a wireguard interface to use the established GnosisVPN connection [Linux]

Create a file called `gnosisvpnpoc.conf` inside `/etc/wireguard/` with the following content.
Replace placeholders `<...>` with the actual values as documented.

```conf
[Interface]
PrivateKey = <Generated automatic by WireGuard app>
ListenPort = 51820
Address = <device IP> # received via drop location, e.g.: 20.0.0.5/32

[Peer]
PublicKey = <wg server pub key> # listed on https://gnosisvpn.com/servers
Endpoint = <hoprd node IP:1422> # port needs to match your `internal_connection_port` configuration
AllowedIPs = 20.0.0.0/24
PersistentKeepalive = 30
```

Activate the WireGuard device with `sudo wg-quick up gnosisvpnpoc`.

### 12. Use GnosisVPN connection to browse the internet [Linux]

For now we only allow SOCKS v5 proxy connections tunneled through GnosisVPN.
The easiest way to do this is to change the Firefox proxy settings.

1. Open Network Connection Settings by navigating into Settings → General → Network Settings or search "proxy" in the settings search bar and click on the "Settings" button.
2. Choose manual proxy configuration and enter:
   - SOCKS Host: `20.0.0.1`
   - Port: `3128`
   - Socks v5
3. Clik "OK" to save the settings.

Start browsing [these select sites](https://gnosisvpn.com/servers#whitelisted) through GnosisVPN.
