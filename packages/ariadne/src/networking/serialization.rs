use super::message::{ClientToServerMessage, ServerToClientMessage};
use either::Either;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SerializationError {
    #[error("Failed to (de)serialize message: {0}")]
    SerializationFailed(#[from] serde_json::Error),

    #[error("Failed to serialize binary message: {0}")]
    BinarySerializationFailed(#[from] ciborium::ser::Error<std::io::Error>),

    #[error("Failed to deserialize binary message: {0}")]
    BinaryDeserializationFailed(#[from] ciborium::de::Error<std::io::Error>),
}

macro_rules! message_serialization {
    ($message_enum:ty $(,$binary_pattern:pat_param)* $(,)?) => {
        impl $message_enum {
            pub fn is_binary(&self) -> bool {
                match self {
                    $(
                        $binary_pattern => true,
                    )*
                    _ => false,
                }
            }

            pub fn serialize(
                &self,
            ) -> Result<Either<String, Vec<u8>>, SerializationError> {
                Ok(match self {
                    $(
                        $binary_pattern => {
                            let mut bytes = Vec::new();
                            ciborium::into_writer(self, &mut bytes)?;
                            Either::Right(bytes)
                        }
                    )*
                    _ => Either::Left(serde_json::to_string(self)?),
                })
            }

            pub fn deserialize(
                msg: Either<&str, &[u8]>,
            ) -> Result<Self, SerializationError> {
                Ok(match msg {
                    Either::Left(text) => serde_json::from_str(&text)?,
                    Either::Right(bytes) => ciborium::from_reader(bytes)?,
                })
            }
        }
    };
}

message_serialization!(
    ClientToServerMessage,
    ClientToServerMessage::SocketSend { .. },
);
message_serialization!(
    ServerToClientMessage,
    ServerToClientMessage::SocketData { .. },
);
