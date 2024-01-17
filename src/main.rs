use base64;

fn main() {
    let original_string = "e3vBu7V8S3s3LwDg5/7d5Pqu+4nA6TGTEAyuf469RbI=";

    // Encode the string
    let encoded_string = encode_to_base64(original_string);
    println!("Encoded String: {}", encoded_string);

    // Decode the string
    let decoded_string = decode_base64_string(&encoded_string);
    println!("Decoded String: {}", decoded_string);
}

fn encode_to_base64(input: &str) -> String {
    base64::encode(input)
}

fn decode_base64_string(encoded_string: &str) -> String {
    match base64::decode(encoded_string) {
        Ok(decoded_bytes) => String::from_utf8(decoded_bytes).expect("Invalid UTF-8"),
        Err(err) => panic!("Error decoding string: {:?}", err),
    }
}
