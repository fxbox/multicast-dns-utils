# Multicast DNS Utils

[![Build Status](https://travis-ci.org/fxbox/multicast-dns.svg?branch=master)](https://travis-ci.org/fxbox/multicast-dns-utils)

Helper tool for Rust [Multicast DNS](https://github.com/fxbox/multicast-dns) crate.

Examples:

To find all http servers exposed via mdns on the network:
```bash
$ cargo run -- -t _http._tcp
+   2   IPv4   DCS-5020L_1D77DA   _http._tcp   local
=   2   IPv4   DCS-5020L_1D77DA   _http._tcp   local
    hostname = [DCS5020L77DA.local]
    address = [192.168.1.3]
    port = [80]
    txt = []
```
