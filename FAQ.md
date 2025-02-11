# FAQ

## Unable to Access Websites After Activating GnosisVPN

If you’ve activated GnosisVPN but cannot open any websites from the [allow list](https://gnosisvpn.com/servers), follow these steps in order to diagnose and resolve the issue:

1. **Check for a Successful VPN Connection**
   After activating GnosisVPN in your terminal, did you see the following success message?

   ```
   /---============================---\
   |   HOPRD CONNECTION ESTABLISHED   |
   \---============================---/
   ```

   - If **yes**, proceed to the next step.
   - If **no**, restart the VPN activation process and ensure there are no errors.
     Double-check that you’ve followed the step-by-step guide carefully to avoid missing any critical steps.

2. **Verify WireGuard is Running and the Tunnel is Active**

   Ensure that WireGuard is running on your machine and that the tunnel has an active status.

   - macOS users: Locate the WireGuard app icon in the top-right corner of the screen, click on "Manage Tunnels", and verify that the tunnel status is "Active".
   - Linux users: Run the command: `sudo wg show`. If the output displays an interface named "gnosisvpnpoc", this indicates that your tunnel is active.

3. **Check for Data Transmission**

   - In the active tunnel, look for the "Data Received:" field.
   - If data is being received, the connection is working.
   - If no data is received, there may be an issue with the VPN connection or your network.

4. **Verify Proxy Settings in Firefox**

   - Ensure the proxy settings in your Firefox browser are configured correctly.
   - Double-check the proxy address and port to match the required settings for GnosisVPN.

5. **Test Website Access**

   If all the above steps are successful, try accessing a website [https://gnosisscan.io](https://gnosisscan.io)