use std::collections::HashMap;

pub fn de_code(code_num: &str) -> u32 {
    let num_rep: HashMap<char, u32> = HashMap::from([
        ('0', 0),
        ('1', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('A', 10),
        ('B', 11),
        ('C', 12),
        ('D', 13),
        ('E', 14),
        ('F', 15),
        ('G', 16),
        ('H', 17),
        ('J', 18),
        ('K', 19),
        ('L', 20),
        ('M', 21),
        ('N', 22),
        ('P', 23),
        ('Q', 24),
        ('R', 25),
        ('S', 26),
        ('T', 27),
        ('U', 28),
        ('V', 29),
        ('W', 30),
        ('X', 31),
        ('Y', 32),
        ('Z', 33),
    ]);
    let mut result: u32 = 0;
    for (index, i) in code_num.chars().rev().enumerate() {
        //println!("{index}");
        result = result + num_rep[&i] * 34_u32.pow(index.try_into().unwrap())
    }
    result
}

/// 生成34进制的数
pub fn gen_code(num: u32) -> String {
    let num_rep: HashMap<i32, char> = HashMap::from([
        (10, 'A'),
        (11, 'B'),
        (12, 'C'),
        (13, 'D'),
        (14, 'E'),
        (15, 'F'),
        (16, 'G'),
        (17, 'H'),
        (18, 'J'),
        (19, 'K'),
        (20, 'L'),
        (21, 'M'),
        (22, 'N'),
        (23, 'P'),
        (24, 'Q'),
        (25, 'R'),
        (26, 'S'),
        (27, 'T'),
        (28, 'U'),
        (29, 'V'),
        (30, 'W'),
        (31, 'X'),
        (32, 'Y'),
        (33, 'Z'),
    ]);

    let mut new_num_string = String::from("");
    let mut current: u32 = num;

    while current != 0 {
        let remainder = (current % 34) as i32;
        let remainder_string: String;

        if remainder > 9 && remainder < 62 {
            remainder_string = format!("{}", num_rep.get(&remainder).unwrap());
        } else {
            remainder_string = format!("{}", remainder);
        }

        new_num_string = format!("{}{}", remainder_string, new_num_string);
        current = current / 34;
    }

    new_num_string
}

/// 10 进制转为 11 - 62 进制 36 进制前是小写
fn base_n(num: u64, n: i32) -> String {
    let num_rep: HashMap<i32, char> = HashMap::from([
        (10, 'a'),
        (11, 'b'),
        (12, 'c'),
        (13, 'd'),
        (14, 'e'),
        (15, 'f'),
        (16, 'g'),
        (17, 'h'),
        (18, 'i'),
        (19, 'j'),
        (20, 'k'),
        (21, 'l'),
        (22, 'm'),
        (23, 'n'),
        (24, 'o'),
        (25, 'p'),
        (26, 'q'),
        (27, 'r'),
        (28, 's'),
        (29, 't'),
        (30, 'u'),
        (31, 'v'),
        (32, 'w'),
        (33, 'x'),
        (34, 'y'),
        (35, 'z'),
        (36, 'A'),
        (37, 'B'),
        (38, 'C'),
        (39, 'D'),
        (40, 'E'),
        (41, 'F'),
        (42, 'G'),
        (43, 'H'),
        (44, 'I'),
        (45, 'J'),
        (46, 'K'),
        (47, 'L'),
        (48, 'M'),
        (49, 'N'),
        (50, 'O'),
        (51, 'P'),
        (52, 'Q'),
        (53, 'R'),
        (54, 'S'),
        (55, 'T'),
        (56, 'U'),
        (57, 'V'),
        (58, 'W'),
        (59, 'X'),
        (60, 'Y'),
        (61, 'Z'),
    ]);

    let mut new_num_string = String::from("");
    let mut current: u64 = num;

    while current != 0 {
        let remainder = (current % (n as u64)) as i32;
        let remainder_string: String;

        if remainder > 9 && remainder < 62 {
            remainder_string = format!("{}", num_rep.get(&remainder).unwrap());
        } else {
            remainder_string = format!("{}", remainder);
        }

        new_num_string = format!("{}{}", remainder_string, new_num_string);
        current = current / (n as u64);
    }

    new_num_string
}

#[cfg(test)]
mod tests {
    use super::de_code;
    use super::gen_code;

    #[test]
    fn test_gen_code() {
        // assert_eq!(get_random_code(), "jdsj")
        let num: u32 = 12000000;
        let code = gen_code(num);
        println!("{num} -> {code}");
        let num2 = de_code("8ZAM6");
        println!("8ZAM6 -> {num2}");
    }
}
