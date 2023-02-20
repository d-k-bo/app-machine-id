# app-machine-id

[![Build Status](https://github.com/d-k-bo/app-machine-id/workflows/CI/badge.svg)](https://github.com/d-k-bo/app-machine-id/actions?query=workflow%3ACI)
[![Crates.io](https://img.shields.io/crates/v/app-machine-id)](https://crates.io/crates/app-machine-id)
[![Documentation](https://img.shields.io/docsrs/app-machine-id)](https://docs.rs/app-machine-id)
[![License: LGPL-2.1-or-later](https://img.shields.io/crates/l/app-machine-id)](LICENSE)

<!-- cargo-rdme start -->

Generate app-specific machine IDs derived from the machine ID defined in
`/etc/machine-id` and an application ID.

Unlike the default machine ID, which should be considered "confidential",
this implementation uses HMAC-SHA256 to generate an app-specific machine ID
which could be used in less secure contexts.

This implementation is based on systemd's `sd_id128_get_machine_app_specific()`.

See [`man machine-id(5)`](https://www.freedesktop.org/software/systemd/man/machine-id.html)
and [`man sd_id128_get_machine(3)`](https://www.freedesktop.org/software/systemd/man/sd_id128_get_machine_app_specific.html)
for details.

<!-- cargo-rdme end -->

## License

This project is licensed under GNU Lesser General Public License version 2.1 or later (LGPL-2.1-or-later).
