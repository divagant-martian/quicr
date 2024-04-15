/// https://datatracker.ietf.org/doc/html/rfc9000#section-2.1-2
/// Between 0 and 2^62
struct Id(u64);

struct OutOfBounds;

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

impl TryFrom<u64> for Id {
    type Error = OutOfBounds;

    fn try_from(id: u64) -> Result<Self, Self::Error> {
        if id > Self::MAX_U64 {
            return Err(OutOfBounds);
        }
        Ok(Id(id))
    }
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
