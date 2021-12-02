use crate::convert::TryFrom;
use crate::fmt;
use crate::io::{self, IoSlice, IoSliceMut};
use crate::net::{Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr};
use crate::str;
use crate::time::Duration;
use super::icecap_impl::Void;

pub struct TcpStream(Void);

impl TcpStream {
    pub fn connect(_: io::Result<&SocketAddr>) -> io::Result<TcpStream> {
        unsupported!()
    }

    pub fn connect_timeout(_: &SocketAddr, _: Duration) -> io::Result<TcpStream> {
        unsupported!()
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        self.0.void()
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        self.0.void()
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.0.void()
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.0.void()
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        self.0.void()
    }

    pub fn read(&self, _: &mut [u8]) -> io::Result<usize> {
        self.0.void()
    }

    pub fn read_vectored(&self, _: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.0.void()
    }

    pub fn write(&self, _: &[u8]) -> io::Result<usize> {
        self.0.void()
    }

    pub fn write_vectored(&self, _: &[IoSlice<'_>]) -> io::Result<usize> {
        self.0.void()
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.0.void()
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        self.0.void()
    }

    pub fn shutdown(&self, _: Shutdown) -> io::Result<()> {
        self.0.void()
    }

    pub fn duplicate(&self) -> io::Result<TcpStream> {
        self.0.void()
    }

    pub fn set_nodelay(&self, _: bool) -> io::Result<()> {
        self.0.void()
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        self.0.void()
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        self.0.void()
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.0.void()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.0.void()
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        self.0.void()
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.void()
    }
}

pub struct TcpListener(Void);

impl TcpListener {
    pub fn bind(_: io::Result<&SocketAddr>) -> io::Result<TcpListener> {
        unsupported!()
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        self.0.void()
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        self.0.void()
    }

    pub fn duplicate(&self) -> io::Result<TcpListener> {
        self.0.void()
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        self.0.void()
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.0.void()
    }

    pub fn set_only_v6(&self, _: bool) -> io::Result<()> {
        self.0.void()
    }

    pub fn only_v6(&self) -> io::Result<bool> {
        self.0.void()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.0.void()
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        self.0.void()
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.void()
    }
}

pub struct UdpSocket(Void);

impl UdpSocket {
    pub fn bind(_: io::Result<&SocketAddr>) -> io::Result<UdpSocket> {
        unsupported!()
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.0.void()
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        self.0.void()
    }

    pub fn recv_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.0.void()
    }

    pub fn peek_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.0.void()
    }

    pub fn send_to(&self, _: &[u8], _: &SocketAddr) -> io::Result<usize> {
        self.0.void()
    }

    pub fn duplicate(&self) -> io::Result<UdpSocket> {
        self.0.void()
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        self.0.void()
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        self.0.void()
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.0.void()
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.0.void()
    }

    pub fn set_broadcast(&self, _: bool) -> io::Result<()> {
        self.0.void()
    }

    pub fn broadcast(&self) -> io::Result<bool> {
        self.0.void()
    }

    pub fn set_multicast_loop_v4(&self, _: bool) -> io::Result<()> {
        self.0.void()
    }

    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        self.0.void()
    }

    pub fn set_multicast_ttl_v4(&self, _: u32) -> io::Result<()> {
        self.0.void()
    }

    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        self.0.void()
    }

    pub fn set_multicast_loop_v6(&self, _: bool) -> io::Result<()> {
        self.0.void()
    }

    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        self.0.void()
    }

    pub fn join_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        self.0.void()
    }

    pub fn join_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        self.0.void()
    }

    pub fn leave_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        self.0.void()
    }

    pub fn leave_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        self.0.void()
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        self.0.void()
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.0.void()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.0.void()
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        self.0.void()
    }

    pub fn recv(&self, _: &mut [u8]) -> io::Result<usize> {
        self.0.void()
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        self.0.void()
    }

    pub fn send(&self, _: &[u8]) -> io::Result<usize> {
        self.0.void()
    }

    pub fn connect(&self, _: io::Result<&SocketAddr>) -> io::Result<()> {
        self.0.void()
    }
}

impl fmt::Debug for UdpSocket {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.void()
    }
}

pub struct LookupHost(Void);

impl LookupHost {
    pub fn port(&self) -> u16 {
        self.0.void()
    }
}

impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> {
        self.0.void()
    }
}

impl TryFrom<&str> for LookupHost {
    type Error = io::Error;

    fn try_from(_v: &str) -> io::Result<LookupHost> {
        unsupported!()
    }
}

impl<'a> TryFrom<(&'a str, u16)> for LookupHost {
    type Error = io::Error;

    fn try_from(_v: (&'a str, u16)) -> io::Result<LookupHost> {
        unsupported!()
    }
}

#[allow(nonstandard_style)]
pub mod netc {
    pub const AF_INET: u8 = 0;
    pub const AF_INET6: u8 = 1;
    pub type sa_family_t = u8;

    #[derive(Copy, Clone)]
    pub struct in_addr {
        pub s_addr: u32,
    }

    #[derive(Copy, Clone)]
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: u16,
        pub sin_addr: in_addr,
    }

    #[derive(Copy, Clone)]
    pub struct in6_addr {
        pub s6_addr: [u8; 16],
    }

    #[derive(Copy, Clone)]
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: u16,
        pub sin6_addr: in6_addr,
        pub sin6_flowinfo: u32,
        pub sin6_scope_id: u32,
    }

    #[derive(Copy, Clone)]
    pub struct sockaddr {}

    pub type socklen_t = usize;
}
