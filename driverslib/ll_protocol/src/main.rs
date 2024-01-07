mod crc;
mod frame;
mod frame_serializer;

/**
* Test main func for sole purpose of testing out LL packet serialization and deserialization
*/
fn main() {
    let payload: Vec<u8> = vec![0x55, 0x66, 0x77, 0x88, 0x99, 0xAA];
    let frame = frame::Frame::new(1, payload);
    let mut frame_serializer = frame_serializer::FrameSerializer::new(frame, true);

    print!("Serialized frame: ");
    while frame_serializer.has_next() {
        print!("{:#03x}, ", frame_serializer.next().unwrap());
    }
}
