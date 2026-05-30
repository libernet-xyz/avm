use crate::proto;
use anyhow::Result;
use starkom_plonk as plonk;

pub trait CircuitFromProto: Sized {
    fn from_proto(message: &proto::Circuit) -> Result<Self>;
}

impl CircuitFromProto for plonk::Circuit {
    fn from_proto(message: &proto::Circuit) -> Result<Self> {
        // TODO
        todo!()
    }
}

pub trait CircuitToProto {
    fn to_proto(&self) -> proto::Circuit;
}

impl CircuitToProto for plonk::Circuit {
    fn to_proto(&self) -> proto::Circuit {
        // TODO
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO
}
