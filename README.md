# pgpdump-rs
[![Build Status](https://travis-ci.org/techwizrd/pgpdump-rs.svg?branch=master)](https://travis-ci.org/techwizrd/pgpdump-rs)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Rust implementation of the PGP packet visualizer. pgpdump-rs displays packets according to the format described in  a PGP packet visualizer which displays packets in the format described by [RFC 4880](https://tools.ietf.org/html/rfc4880) and [RFC 1991](https://tools.ietf.org/html/rfc1991). This is also a library that you can embed in your own projects to parse PGP packets in binary/ASCII-armored representations.

This is heavily inspired by the following projects:
* the original [C implementation of `pgpdump`](http://www.mew.org/~kazu/proj/pgpdump/en/) by Kazu Yamamoto
* [python-pgpdump](https://github.com/toofishes/python-pgpdump) by [Dan McGee](https://github.com/toofishes)

### Usage

Include the following in your `Cargo.toml`
```toml
[dependencies]
pgpdump = { git = "https://github.com/srct/pgpdump-rs" }
```

You can then use it in your projects using
```rust
extern crate pgpdump;
use pgpdump::PgpDump;
use std::str::FromStr;

fn main() {
    let ascii_data: &'static str ="-----BEGIN PGP PUBLIC KEY BLOCK-----
mQENBFXl288BCADOLAwf89kuek78E66ek+AXD8jojlaFoJcoO+Du51m8XJIWDzAF
Gnvff5NDZhqNgpC28CYHkzekfcNq3rXxeGx610a1zKI2ZRwk6ogvLWa5UoTVsWHy
712zwtLtlt7Y6AMdFyPNuSNq+dKc+6WX0R3N+kaaUYw4GnWQLi+odChkWnwklsvJ
4i4Xd8dC5U27SqNYwsva+01W8FEFlvUg+yKzkrMBtJW0NhFbzA068SAurISjXcWJ
2tbjBC+HSD8nFdECeXBIymhx/nMizjHSczpu17J7R2p6QKK5cUaPhTI+50PrOjtj
See5InYVTOA76FIvde6goNOZNh/N5IMc6uoJABEBAAG0J0t1bmFsIFNhcmtoZWwg
KFNSQ1QpIDxrc2Fya2hlbEBnbXUuZWR1PokBPgQTAQIAKAIbAwYLCQgHAwIGFQgC
CQoLBBYCAwECHgECF4AFAlmx/lwFCQeOiY0ACgkQ1VSIBRbOAvmuuQf/TNrtt+pm
JvGz1OcTi4beuGmu2p/Q3MUzGsio4aLit5uc1pMTK0Hp+laCjI8vIZ1xnhiruM2P
4n9qhVmZR13efMyQdxrgdwhgzpwbIBMcVyAkU+N2z27YpORS9KsGalw+6L64FXrb
9CbE4VGI1r6ivRKlbW1HSsHXcyGKrdQRi0KMpiL7CRARyo8TDOgXzVIXNskvxHN+
rQs+V8Cry+T1jeP7lTDFcWt//sMkMjNhDwMrHsEI8CRk24F0ZS1xFuGOpPXSEMQf
gmxg79CwtAEQ29FNi+VFrbcfZMxAX3/UAialKEPvpqmAA92JMwA/hMNd2dhdc+sZ
FpBFPf7g9MsZ57kBDQRV5dvPAQgA6HNlKpU1fQLmbzKre3hHXYpJx72amSKanu92
K3nRjnp4h68Va57GY9S61hlwGEO8mBeY1FtaLNke+nGBo5WLS2noqLiFkSa6w481
0wdWd56oxq2zKoxceu19Gv5nAGI1AGpI56PkxXdsPDB7vyIcrVEUvx0MI/Msouig
SgQgECd/e+yv4R8i+ltlnzPIKO1zKbNN7fpBY7NQPyHSwEEJztqFDlV7u5eN6pYY
3QK2+B73FH8DmMoV+1q3RhR+NHm5TLqep84jJsodUZCMmq7aIo9KkXgiNVkCnRS2
Yu8ThQPmOZcK0IsqVcr7h7KS85gfS8PFiaj3SuncNR7zU3fxewARAQABiQElBBgB
AgAPAhsMBQJZsf63BQkHjonoAAoJENVUiAUWzgL5XywH/jVPycU/vkqzmJeYYzE6
5G6lsWiuo/Vj6echO5lp7KFbt534aQTM+qRCNAmmyH5TzIRuun75UgAAd/EIjnR/
e+mZhljdxOCQ/SN4HtdHj7WId6cn5qWbRIj5BiIcHriCb+youcvEsp1ewZHQmgGy
i4G7P0hrnAXRDLUjXePxflGg2yBUX64j0sYG2vhRE54QJsygkZ14ff+XpILv+/hf
Xe0s419jMp+1TyU58DPfTU+cvIiYN1IgmT9hbLYoCdbTj9n9xK04rRB6c9HeFxph
yc6Ir9FF4S8r4HCv+sfPg85HEBDpDeCXoyWcE7c0IFONcrqsb0MnINftC90NCjQ6
vuc=
=FHic
-----END PGP PUBLIC KEY BLOCK-----";
    let _packets = PgpDump::from_ascii(data).unwrap().get_packets();
}
```

For more examples, see the `examples/` folder.

### CHANGELOG

Please see the [CHANGELOG](CHANGELOG.md) for a release history.

