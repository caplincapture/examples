enum IpAddrKind {
    V4(),
    V6(),
}

mod protocolOne {
    #[derive(Clone)]
    pub(crate) struct IpAddrKind;
}

mod protocolTwo {
    pub(crate) struct IpAddrKind;
}

// private module, lest outside users invent their own protocol kinds!
mod Proto_Trait {

    use super::{protocolOne, protocolTwo, IpAddrKind};

    pub(crate) trait PrototypeProtocol {
        type IpAddrKind;
    }

    pub struct ProtocolOne {
        prot: IpAddrKind,
    }

    impl ProtocolOne {
        pub(crate) fn mount_ip(&self) -> IpAddrKind {
            self.prot
        }
    }

    impl PrototypeProtocol for ProtocolOne {
        type IpAddrKind = protocolOne::IpAddrKind;
    }

    pub struct ProtocolTwo {
        pub(crate) prot: IpAddrKind,
    }

    impl PrototypeProtocol for ProtocolTwo {
        type IpAddrKind = protocolTwo::IpAddrKind;
    }
}
 // keep internal to prevent impls
pub use Proto_Trait::{ProtocolOne, ProtocolTwo};

use self::{Proto_Trait::PrototypeProtocol};

struct Process<P: PrototypeProtocol> {
    protocol: P,
}

// all common API parts go into a generic impl block
impl Process<ProtocolOne> {
    fn mount_ip(&self) -> IpAddrKind {
        self.protocol.mount_ip()
    }
}

use crate::libfunc::IpAddrKind::V6;
// all protocol-specific impls go into their own block

fn main() {
    let proc = Process{protocol: ProtocolTwo{prot: V6()}};
    proc.mount_ip()
}
