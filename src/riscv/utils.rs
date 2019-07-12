pub fn combine_parameters(args: Vec<String>) -> Vec<u8> {
    let mut r: Vec<u8> = Vec::new();
    for e in &args {
        let mut a = e.as_bytes().to_owned();
        r.append(&mut a);
        r.push(0x00);
    }
    r
}
