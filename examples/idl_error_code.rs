fn main() {
    let res = build_code(2, 1);
    println!("res: {:?}", res);

    let (id, idx) = parse_code(res);
    println!("id: {}, idx: {}", id, idx);
}

fn build_code(app_id: u16, idx: u16) -> u16 {
    let err_code = ((app_id as u16) << 8) | idx;
    err_code
}

fn parse_code(err_code: u16) -> (u16, u16) {
    let app_id = err_code >> 8;
    let idx = err_code & 0xFF;

    (app_id, idx)
}
