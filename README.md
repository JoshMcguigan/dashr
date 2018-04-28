# Dashr

Dashr was originally intended to be used to trigger events from button presses of Amazon's [Dash](https://en.wikipedia.org/wiki/Amazon_Dash) buttons ([as shown here](https://blog.cloudstitch.com/how-i-hacked-amazon-s-5-wifi-button-to-track-baby-data-794214b0bdd8)). Rather than hard coding specific actions that should be taken when the Dash button is pressed, Dashr follows the Unix philosophy, and simply writes the MAC address of the device to stdout. As shown in the usage examples below, this allows for tremendous flexibility.

A couple of potential smart-home ideas for Dashr include turning off lights when a Dash button is pressed, or turning down the air conditioner when you get home (by listening for the DHCP request from your cell phone reconnecting to wifi).

### Install & Use

Run ifconfig to determine the name of the network adapter you want to use to listen for traffic. The example below assumes 'en0'.

```
cargo install
sudo dashr en0
```

On OS X, the following command will say (through the speakers) the MAC address of any device requesting an IP address via DHCP.

```
sudo dashr en0 | while read line ; do echo $line | say ; done
```

The following command will run `script.sh` anytime the device with MAC `88:71:E5:24:C4:76` requests an IP address via DHCP.

```
sudo dashr en0 | grep --line-buffered 88:71:E5:24:C4:76 | while read line ; do echo $line | ./script.sh ; done
```

With [hue-cli](https://github.com/JoshMcguigan/hue-cli) it is possible to toggle the status (on/off) of you Hue lights by pressing the Dash button.

```
sudo dashr en0 | grep --line-buffered 88:71:E5:24:C4:76 | while read line ; do echo $line | hue-cli cmd toggle ; done
```

If you'd rather not install dashr, you can replace `sudo dashr` above with `sudo cargo run`. In OS X, `sudo` is required to listen for network traffic.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

### Attributions

This code is based heavily on the [packet dump](https://github.com/libpnet/libpnet/blob/89b7f5f916f109fe8588e02eea0bfcc6fdb84b90/examples/packetdump.rs) example of the [libpnet](https://github.com/libpnet/libpnet) library. 
