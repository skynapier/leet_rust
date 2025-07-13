
fn main() {
    let s = String::from("ABCD");
    let len = s.len();
    let num_rows = 3;

    let s_bytes = s.as_bytes();
    let mut result = Vec::new();
    let cycle_len = 2 * (num_rows - 1);

    for row in 0..num_rows {
        let mut pos = row;

        while pos < len {
            result.push(s_bytes[pos]);

            if row != 0 && row != num_rows - 1 {
                let diagonal_pos = pos + cycle_len - 2 * row;
                if diagonal_pos < len {
                    result.push(s_bytes[diagonal_pos]);
                }
            }

            pos += cycle_len;
        }
    }

    println!("{:?}",  String::from_utf8(result).unwrap());
}
