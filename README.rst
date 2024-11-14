===============
 roman-numerals
===============

.. image:: https://img.shields.io/pypi/v/roman-numerals-py.svg
   :target: https://pypi.org/project/roman-numerals-py/
   :alt: Package on PyPI

.. image:: https://img.shields.io/crates/v/roman-numerals-rs
   :target: https://crates.io/crates/roman-numerals-rs/
   :alt: Package on Crates.io

.. image:: https://img.shields.io/badge/Licence-0BSD-green.svg
   :target: https://opensource.org/license/0BSD
   :alt: Licence: 0BSD

.. image:: https://img.shields.io/badge/License-Public%20Domain-green.svg
   :alt: Licence: Public Domain

This project provides utilities manipulating well-formed Roman numerals,
in various programming languages.
Currently, there are implementations in Python__ and Rust__.

__ ./python/README.rst
__ ./rust/README.md

Example usage
=============

Rust
----

.. code-block:: rust

   use roman_numerals_rs::RomanNumeral;

   fn main() {
      let num = RomanNumeral::new(16);
      println!("{}", num); // XVI
      assert_eq!("XVI".parse().unwrap(), num);
   }


Python
------

.. code-block:: python

   from roman_numerals import RomanNumeral

   num = RomanNumeral(16)
   print(num)  # XVI
   assert RomanNumeral.from_string("XVI") == num


Licence
=======

This project is placed in the public domain
or under the terms of the `'Zero Clause BSD licence'`__,
whichever is more permissive.

__ ./LICENCE.rst
