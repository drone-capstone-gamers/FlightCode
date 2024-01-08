mod crc;
pub mod frame;
pub mod frame_serializer;
pub mod frame_deserializer;

#[cfg(test)]
mod ll_protocol_tests {
    use std::ops::Deref;
    use crate::{frame, frame_deserializer, frame_serializer};

    #[test]
    fn serialize_and_deserialize() {
        let payload: Vec<u8> = vec![0x55, 0x66, 0x77, 0x88, 0x99, 0xAA];
        let frame = frame::Frame::new(1, payload.clone());
        let frame_serializer = frame_serializer::FrameSerializer::new(frame, true);

        let serialized_frame =  &frame_serializer.collect::<Vec<u8>>();

        print!("Serialized frame: ");
        for byte in serialized_frame {
            print!("{:#03x}, ", byte);
        }
        let expected_serialized: Vec<u8> = vec![0x55, 0x46, 0xaa, 0x5, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xa, 0x1b, 0x55];
        assert_eq!(serialized_frame.deref(), expected_serialized);

        let frame = frame::Frame::new(1, payload);
        let mut frame_deserializer = frame_deserializer::FrameDeserializer::new();
        let _ = serialized_frame.iter().map(|&byte| frame_deserializer.apply(byte))
            .filter(|result| result.is_some())
            .for_each(|result| {
                let deserialized_frame = result.unwrap();
                print!("\n\nDeserialized frame: {}", deserialized_frame);
                assert_eq!(deserialized_frame, frame);
            });
    }
}
