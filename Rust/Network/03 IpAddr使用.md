### IpAddr

IpAddr 是一个枚举类型，包含 V4 和 V6 两种版本

```rust
pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

`Ipv4Addr` 和 `Ipv6Addr` 定义新对象直接输入整型

```rust
let v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
let v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
```

也可通过字符串解析，`str` 和 `String` 皆可

```rust
assert_eq!(format!("127.0.0.1").parse(), Ok(v4));
assert_eq!("127.0.0.1".parse(), Ok(v4));
assert_eq!("::1".parse(), Ok(v6));
```

环回地址和广播地址

```rust
assert_eq!(v4.is_loopback(), true);
assert_eq!(v6.is_loopback(), true);
assert_eq!("224.0.0.0".parse().is_multicast(), true);
```

此外还定义了一些常量，如 `LOCALHOST`

```rust
assert_eq!("127.0.0.1".parse(), Ok(IpAddr::V4(Ipv4Addr::LOCALHOST)));
```

