use std::io::{self, Write};

//per multiple of 20 price increase by 5
const PER_20_KM_PRICE_INCREASE: i8 = 5;
const BASE_DISTANCE_PRICE: i8 = 5;
const BASE_DISTANCE: i8 = 20;
#[derive(Debug)]
struct Platform<'a> {
    name: &'a str,
    code: i8,
    distance: i8, //distance from main platform
}

impl<'a> Platform<'a> {
    fn new(name: &'a str, code: i8, distance: i8) -> Self {
        Platform {
            name,
            code,
            distance,
        }
    }
    //calculate distance between 2 platforms, does not consider intermediate platforms
    fn distance(self: &Self, platform: &Platform) -> i8 {
        let difference = self.distance - platform.distance;
        difference.abs()
    }
    //calculate price on basis of distance
    fn price(self: &Self, platform: &Platform) -> i16 {
        let distance = self.distance(platform);
        let base_distance_multiply = distance / BASE_DISTANCE; //divide distance by base distance
        let price =
            BASE_DISTANCE_PRICE as i16 + (base_distance_multiply * PER_20_KM_PRICE_INCREASE) as i16;
        price
    }
}

fn main() -> io::Result<()> {
    //launch the program
    println!("Welcome to Simple ticket.");

    //let create a new station
    let platform_0 = Platform::new("Howrah", 0, 0);
    let platform_1 = Platform::new("Liluah", 1, 5);
    let platform_2 = Platform::new("Belur", 2, 6);
    let platform_3 = Platform::new("Bally", 3, 10);
    let platform_4 = Platform::new("Uttarpara", 4, 12);
    let platform_5 = Platform::new("Konnager", 5, 13);
    let platform_6 = Platform::new("Rishra", 6, 15);
    let platform_7 = Platform::new("Serampore", 7, 17);

    let platforms = [
        platform_0, platform_1, platform_2, platform_3, platform_4, platform_5, platform_6,
        platform_7,
    ];
    pretty_print_platforms(&platforms);

    print!("Enter current station     : ");
    io::stdout().flush()?;
    let current_platform_code = get_input().expect("Enter number");

    //get the current station by code from platforms
    let current_platform = platforms
        .iter()
        .find(|&x| x.code as i32 == current_platform_code);

    match current_platform {
        Some(cs) => {
            print!("Enter destination station : ");
            io::stdout().flush()?;
            let destination_platform_code = get_input().expect("Enter number");
            let destination_platform = platforms
                .iter()
                .find(|&x| x.code as i32 == destination_platform_code);
            match destination_platform {
                Some(ds) => {
                    print!("Enter unit : ");
                    io::stdout().flush()?;
                    let unit = get_input().unwrap_or_else(|_x| 0);
                    let price = cs.price(ds) as i32 * unit;
                    println!("Pay {} for {} person", price, unit);
                }
                None => {
                    println!("No destination found")
                }
            }
        }
        None => {
            println!("No current station found")
        }
    };

    Ok(())
}

//take slice of platforms an print them,
fn pretty_print_platforms(platforms: &[Platform]) {
    //print on 3 columns , show platform code and platform name 6 digits
    let mut count = 0;
    let mut plat_ite = platforms.iter();
    loop {
        match plat_ite.next() {
            Some(platform) => {
                print!("{} : {:?}\t", platform.code, platform.name);
                count = count + 1;
            }
            None => {
                println!("");
                break;
            }
        }
        if count == 3 {
            count = 0;
            println!("")
        }
    }
}

//read input from stdinput and return number
fn get_input() -> io::Result<i32> {
    let mut current_platform_code: String = String::new();
    io::stdin().read_line(&mut current_platform_code)?;
    let current_platform_code: i32 = current_platform_code.trim().parse().unwrap();
    Ok(current_platform_code)
}
