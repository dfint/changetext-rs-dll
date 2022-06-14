fn main() {
    let mut vector = Vec::<u8>::new();
    let bytes = b"ab";
    // vector.copy_from_slice(bytes);
    // vector.copy_from_slice(bytes);
    vector.push(bytes[0]);
    vector.push(bytes[1]);
    
    println!("{vector:?}");
}
