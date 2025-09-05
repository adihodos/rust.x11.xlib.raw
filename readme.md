# X11 application from scratch, using Rust

## References
- [X.Org Foundation](https://www.x.org/wiki/)
- [X11 programming guides](https://www.x.org/releases/X11R7.6/doc/)
- [Rust Core library](https://doc.rust-lang.org/core/)
- [XCB Api reference](https://xcb.freedesktop.org/manual/group__XCB____API.html)
- [libc reference](https://man7.org/linux/man-pages/man7/libc.7.html)

## Build steps
```
rustc -o playground playground.rs && ./playground
rustc -o playground.xcb rust.xcb.rs && ./playground.xcb.rs
```

## Description
- the first app demonstrates classix X11 event handling
- second app uses xcb api and non blocking I/O (epoll events) to read xcb events from the xcb connection’s file descriptor
