Release 4.1.0 (released 17 Dec 2025)
====================================

* Restore LICENCE.rst in the roman-numerals-py meta-package
* Add some debug assertions for unsafe blocks

Release 4.0.0 (released 16 Dec 2025)
====================================

* Rename Python package to ``roman-numerals`` on PyPI.
* Publish ``roman-numerals-py`` as a meta-package that depends
  on ``roman-numerals`` and installs no modules.
* Drop support for Python 3.9.
* Declare support for Python 3.15.
* Increase the minimum supported Rust version (MSRV) to 1.81.0.
* Implement the ``core::error::Error`` trait for all error types.
* Implement the ``core::fmt`` traits in ``no-std`` mode.
* Implement ``From<RomanNumeral>`` for most integer types.
* Publish the ``roman-numerals-rs`` crate using `Trusted Publishing
  <https://crates.io/docs/trusted-publishing>`__

Release 3.1.0 (released 22 Feb 2025)
====================================

* Support flit-core 3.11.
* Increase the minimum supported Rust version (MSRV) to 1.79.0.

Release 3.0.0 (released 18 Feb 2025)
====================================

* Remove runtime imports from ``typing``.
* Declare support for Python 3.14.
* Implement ``RomanNumeral`` as a ``NonZero<u16>`` tuple-struct.
* Add ``no-std`` support for the ``roman-numerals-rs`` crate.

Release 2.0.0 (released 14 Nov 2024)
====================================

Dual-licence under either the Zero-Clause BSD or the CC0-1.0 Universal licence.

Release 1.0.0 (released 10 Nov 2024)
====================================

First release.
