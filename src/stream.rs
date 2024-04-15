#![allow(unused)]
struct Stream {
    id: Id,
    // TODO(@divma): type?
    // offset: u8
}

impl Stream {
    /* Sending side */
    fn write(&self, data: &[u8]) -> Result<(), ()> {
        Ok(())
    }
    fn end(&self) {}
    fn reset(&self) {}
    /* Receiving side */
    fn read(&self) -> Vec<u8> {
        vec![]
    }
    fn abort(&self) {}
}

/// https://datatracker.ietf.org/doc/html/rfc9000#section-2.1-2
/// Between 0 and 2^62
struct Id(u64);

impl Id {
    const BITS: u8 = 62;
    const MAX_U64: u64 = 2u64.pow(Self::BITS as u32);
    const MAX: Id = Id(Self::MAX_U64);

    const fn initiator(&self) -> Initiator {
        // check if the server's bit is set
        if self.0 & Initiator::SERVER_BIT == Initiator::SERVER_BIT {
            Initiator::Server
        } else {
            Initiator::Client
        }
    }

    const fn kind(&self) -> Kind {
        // check if the unidirectional's bit is set
        if (self.0 & Kind::UNIDIR_BIT) == Kind::UNIDIR_BIT {
            Kind::Unidirectional
        } else {
            Kind::Bidirectional
        }
    }
}

/// [`Id`] is larger than [`Id::MAX`].
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(fmt = "Id is out of bounds")]
struct IdOutOfBoundsErr;

impl TryFrom<u64> for Id {
    type Error = IdOutOfBoundsErr;

    fn try_from(id: u64) -> Result<Self, Self::Error> {
        if id > Self::MAX_U64 {
            return Err(IdOutOfBoundsErr);
        }
        Ok(Id(id))
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Initiator {
    Client,
    Server,
}

impl Initiator {
    const SERVER_BIT: u64 = 0x1;
}

#[derive(Debug, PartialEq, Eq)]
enum Kind {
    Unidirectional,
    Bidirectional,
}

impl Kind {
    const UNIDIR_BIT: u64 = 0x02;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test Table from https://datatracker.ietf.org/doc/html/rfc9000#stream-id-types
    #[test]
    fn test_client_initiated() {
        let cases = [
            (0x00, Initiator::Client, Kind::Bidirectional),
            (0x01, Initiator::Server, Kind::Bidirectional),
            (0x02, Initiator::Client, Kind::Unidirectional),
            (0x03, Initiator::Server, Kind::Unidirectional),
        ];
        for (id, expected_initiator, expected_kind) in cases {
            let id = Id(id);
            let kind = id.kind();
            let initiator = id.initiator();
            assert_eq!(kind, expected_kind);
            assert_eq!(initiator, expected_initiator);
        }
    }
}
