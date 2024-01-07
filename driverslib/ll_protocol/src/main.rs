mod crc;
mod frame;
mod frame_serializer;

/**
* Test main for sole purpose of testing out LL packet serialization and deserialization
*/
fn main() {
    let payload: Vec<u8> = vec![0x55, 0x66, 0x77, 0x88, 0x99, 0xAA];
    let frame = frame::Frame::new(1, payload);
    let mut frameSerializer = frame_serializer::FrameSerializer::new(frame, true);

    print!("Serialized frame: ");
    while frameSerializer.hasNext() {
        print!("{:#03x}, ", frameSerializer.next().unwrap());
    }
}
