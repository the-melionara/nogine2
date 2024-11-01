use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteSize {
    bytes: u64,
}

impl ByteSize {
    pub const fn new(bytes: u64) -> Self {
        Self { bytes }
    }

    pub const fn as_bytes(&self) -> u64 {
        self.bytes
    }

    pub const fn as_kilobytes(&self) -> u64 {
        self.bytes / 1000
    }

    pub const fn as_kibibytes(&self) -> u64 {
        self.bytes >> 10
    }

    pub const fn as_megabytes(&self) -> u64 {
        self.bytes / 1_000_000
    }

    pub const fn as_mebibytes(&self) -> u64 {
        self.bytes >> 20
    }

    pub const fn as_gigabytes(&self) -> u64 {
        self.bytes / 1_000_000_000
    }

    pub const fn as_gibibytes(&self) -> u64 {
        self.bytes >> 30
    }

    pub const fn as_terabytes(&self) -> u64 {
        self.bytes / 1_000_000_000_000
    }

    pub const fn as_tebibytes(&self) -> u64 {
        self.bytes >> 40
    }

    pub const fn as_petabytes(&self) -> u64 {
        self.bytes / 1_000_000_000_000_000
    }

    pub const fn as_pebibytes(&self) -> u64 {
        self.bytes >> 50
    }
}

impl Add for ByteSize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { bytes: self.bytes + rhs.bytes }
    }
}
