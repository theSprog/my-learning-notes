### String，＆str，Vec\<u8>和＆[u8]的惯用转换

```rust
&str    -> String--| String::from(s) or s.to_string() or s.to_owned()

&str    -> &[u8]---| s.as_bytes()

&str    -> Vec<u8>-| s.as_bytes().to_vec() or s.as_bytes().to_owned()

String  -> &str----| &s if possible* else s.as_str()

String  -> &[u8]---| s.as_bytes()

String  -> Vec<u8>-| s.into_bytes()

&[u8]   -> &str----| std::str::from_utf8(s).unwrap()

&[u8]   -> String--| String::from_utf8(s).unwrap()

&[u8]   -> Vec<u8>-| &s if possible* else s.as_slice()

Vec<u8> -> &str----| std::str::from_utf8(&s).unwrap()

Vec<u8> -> String--| String::from_utf8(s).unwrap()

Vec<u8> -> &[u8]---| as_slice()
```