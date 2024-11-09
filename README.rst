===============
 roman-numerals
===============

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
