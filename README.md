# knx_examples_rs

[![Apache licensed](https://img.shields.io/badge/license-Apache-blue.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![Actions Status](https://github.com/uwearzt/knx_examples_rs/workflows/push_pullreq/badge.svg)](https://github.com/uwearzt/knx_examples_rs/actions)
[![crates.io](https://meritbadge.herokuapp.com/knx_examples_rs)](https://crates.io/crates/knx_examples_rs)
[![Gitter](https://badges.gitter.im/knx_rs/Lobby.svg)](https://gitter.im/knx_rs/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

The `knx_examples_rs` implements some examples how
to use the `knx_rs` crate.

## knx_listen

Bus Monitor for logging all KNX bus messages:

```sh
knx_listen --serial --serialport /dev/usb_to_knx
knx_listen --multicast
```

For getting group and address names instead of the numerical values, it is possible to load a OPC file
into knx_listen.

```sh
knx_listen --opcfile opc/Haus.esf.utf8 --multicast
```

You can export the OPC File in ETS with:

after that you have to convert the file with:

```sh
iconv -f ISO-8859-15 -t UTF-8 Haus.esf > Haus.esf.utf8
```

## knx_send

Send a  message to a group adress:

```sh
knx_send --serial --serialport /dev/usb_to_knx 1/1/1 1
knx_send --multicast 1/1/1 1
```

## Contributors

* mail@uwe-arzt.de Uwe Arzt

## License

[Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
