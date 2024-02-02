use std::fs::read_to_string;

static RED_CUBES: i32 = 12;
static GREEN_CUBES: i32 = 13;
static BLUE_CUBES: i32 = 14;

fn main() {
    let mut sum_valid: i32 = 0;
    let mut sum_powers: i32 = 0;

    for line in read_to_string("src/input").unwrap().lines() {
        let mut is_valid = true;
        let (mut min_red, mut min_blue, mut min_green) = (0, 0, 0);
        let info: Vec<&str> = line.split(":").collect();
        let id = info[0].split(" ").collect::<Vec<_>>()[1]
            .parse::<i32>()
            .expect("Failed to parse game_id");

        info[1].split(";").for_each(|reveal| {
            reveal.split(",").for_each(|color| {
                let c = color.trim().split(" ").collect::<Vec<_>>();
                let count = c[0].parse::<i32>().expect("Failed to parse count");
                let name = c[1];

                if name == "red" && count > min_red {
                    min_red = count;
                } else if name == "blue" && count > min_blue {
                    min_blue = count;
                } else if name == "green" && count > min_green {
                    min_green = count;
                }

                let valid_amount = match name {
                    "red" => count <= RED_CUBES,
                    "green" => count <= GREEN_CUBES,
                    "blue" => count <= BLUE_CUBES,
                    _ => false,
                };
                is_valid = is_valid && valid_amount;
            })
        });

        if is_valid {
            sum_valid += id;
        }

        let power: i32 = min_red * min_blue * min_green;
        sum_powers += power;
    }

    println!("valid: {}, power: {}", sum_valid, sum_powers);
}
