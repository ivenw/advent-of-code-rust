use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, Clone, Eq)]
pub enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Int(a) => write!(f, "{}", a),
            Packet::List(a) => {
                write!(f, "[")?;
                for (idx, item) in a.iter().enumerate() {
                    write!(f, "{}", item)?;
                    if idx != a.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::Int(a), Packet::List(b)) => vec![Packet::Int(*a)].cmp(b),
            (Packet::List(a), Packet::Int(b)) => a.cmp(&vec![Packet::Int(*b)]),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a == b,
            (Packet::List(a), Packet::List(b)) => a == b,
            (Packet::Int(a), Packet::List(b)) => &vec![Packet::Int(*a)] == b,
            (Packet::List(a), Packet::Int(b)) => a == &vec![Packet::Int(*b)],
        }
    }
}
