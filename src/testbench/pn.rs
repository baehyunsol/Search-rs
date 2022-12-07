use hserde::HSerde;

const DELIMITER: u8 = '|' as u8;

// n must be an odd number!
fn is_pn(n: u32) -> bool {
    let mut div = 3;

    while div * div <= n {

        if n % div == 0 {
            return false;
        }

        div += 2;
    }

    true
}

// including an ending delimiter
fn num_to_bytes(n: u32) -> Vec<u8> {
    let mut result = Vec::with_capacity(8);

    for b in n.to_bytes() {
        result.push(b);
    }

    result.push(DELIMITER);

    result
}

fn pn_gen() {
    use crate::file::write_to_file;

    let mut result = vec![DELIMITER];

    for b in 2u32.to_bytes() {
        result.push(b);
    }

    for i in 1..0x20_000 {
        let n = i * 2 + 1;

        if !is_pn(n) {
            continue;
        }

        else {

            for b in num_to_bytes(n) {
                result.push(b);
            }

        }

    }

    write_to_file("0.txt", &result).unwrap();

    for s in 1..256 {
        result = vec![DELIMITER];

        for i in (0x20_000 * s)..(0x20_000 * s + 0x20_000) {
            let n = i * 2 + 1;

            if !is_pn(n) {
                continue;
            }

            else {

                for b in num_to_bytes(n) {
                    result.push(b);
                }

            }

        }

        write_to_file(&format!("{}.txt", s), &result).unwrap();
    }

}