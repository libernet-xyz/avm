use crate::version::AVM_MAJOR_VERSION;
use crate::{proto, version::AVM_MINOR_VERSION};
use anyhow::{Context, Result, anyhow};
use starkom_bluesky::Scalar;
use starkom_plonk as plonk;

pub trait ScalarFromProto: Sized {
    fn from_proto(message: &proto::Scalar) -> Result<Self>;
}

impl ScalarFromProto for Scalar {
    fn from_proto(message: &proto::Scalar) -> Result<Self> {
        if message.value.len() != 32 {
            return Err(anyhow!(
                "invalid scalar length: expected 32 bytes, got {}",
                message.value.len()
            ));
        }
        Scalar::from_little_endian(&message.value)
            .into_option()
            .context("parsed scalar is out of the BlueSky range")
    }
}

pub trait ScalarToProto {
    fn to_proto(&self) -> proto::Scalar;
}

impl ScalarToProto for Scalar {
    fn to_proto(&self) -> proto::Scalar {
        proto::Scalar {
            value: self.to_little_endian().to_vec(),
        }
    }
}

pub trait CircuitFromProto: Sized {
    fn from_proto(message: &proto::Circuit) -> Result<Self>;
}

impl CircuitFromProto for plonk::Circuit {
    fn from_proto(message: &proto::Circuit) -> Result<Self> {
        match message.version.as_ref() {
            Some(version) => {
                if version.major != AVM_MAJOR_VERSION {
                    return Err(anyhow!(
                        "unsupported circuit version: expected major version {}, got {}",
                        AVM_MAJOR_VERSION,
                        version.major
                    ));
                }
                if version.minor < AVM_MINOR_VERSION {
                    return Err(anyhow!(
                        "unsupported circuit version: expected minor version >={}, got {}",
                        AVM_MINOR_VERSION,
                        version.minor
                    ));
                }
            }
            None => return Err(anyhow!("missing circuit version")),
        };

        let n = message.gates.len();

        let mut ql = Vec::with_capacity(n);
        let mut qr = Vec::with_capacity(n);
        let mut qo = Vec::with_capacity(n);
        let mut qm = Vec::with_capacity(n);
        let mut qc = Vec::with_capacity(n);

        for gate in &message.gates {
            ql.push(Scalar::from_proto(
                gate.ql.as_ref().context("invalid gate: missing ql")?,
            )?);
            qr.push(Scalar::from_proto(
                gate.qr.as_ref().context("invalid gate: missing qr")?,
            )?);
            qo.push(Scalar::from_proto(
                gate.qo.as_ref().context("invalid gate: missing qo")?,
            )?);
            qm.push(Scalar::from_proto(
                gate.qm.as_ref().context("invalid gate: missing qm")?,
            )?);
            qc.push(Scalar::from_proto(
                gate.qc.as_ref().context("invalid gate: missing qc")?,
            )?);
        }

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
