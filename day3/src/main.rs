use regex::Regex;
use std::fs::read_to_string;

fn is_symbol(ch: &char) -> bool {
    let symbol_regex = Regex::new(r"[^A-Za-z0-9\.\n\s]").unwrap();
    return symbol_regex.is_match(ch.to_string().as_str());
}

fn is_part_number(start: usize, end: usize, src: &Vec<char>, line_len: usize) -> bool {
    // TODO: refactor
    let mut adjacent = if start > 0 {
        vec![start - 1, end]
    } else {
        vec![end + 1]
    };
    let s = if start > 0 { start - 1 } else { start };

    for i in s..=end {
        adjacent.push(i + line_len);

        if i > line_len {
            adjacent.push(i - line_len);
        }
    }

    for adj in adjacent {
        let ch = src.get(adj);

        if ch.is_some() {
            if is_symbol(ch.unwrap()) {
                return true;
            }
        }
    }

    return false;
}

fn main() {
    let nl_regex = Regex::new(r"\n").unwrap();
    let number_regex = Regex::new(r"[0-9]+").unwrap();
    let file = read_to_string("src/input").unwrap();
    let input_str = file.as_str();
    let line_len = nl_regex.find(input_str).unwrap().end();
    let mut sum = 0;

    for num in number_regex.find_iter(input_str) {
        let input_chars = file.chars().collect::<Vec<_>>();
        let is_pn = is_part_number(num.start(), num.end(), &input_chars, line_len);

        if is_pn {
            let str: String = input_chars[num.start()..num.end()]
                .to_vec()
                .into_iter()
                .collect();
            let num: usize = str.parse().unwrap();
            print!("num: {}", num);

            sum += num;
        }
    }

    print!("sum: {}", sum);
}
