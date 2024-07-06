use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Open input file
    if let Ok(lines) = read_lines("./seeds.txt") {
        // Process lines in file
        for line in lines.flatten() {
            // Do stuff with file contents
            let seeds = line.split_whitespace().collect::<Vec<&str>>();
            let mut closest : u64 = 0;
            let mut answer : u64 = 0;
            for seed in seeds {
                let seed = seed.parse::<u64>().unwrap();
                let soil = map_search("./seeds-soil.txt", seed);
                if soil != 0 {
                    let fertilizer = map_search("./soil-fertilizer.txt",soil);
                    if fertilizer != 0 {
                        let water = map_search("./fertilizer-water.txt",fertilizer);
                        if water != 0 {
                            let light = map_search("./water-light.txt",water);
                            if light != 0 {
                                let temperature = map_search("./light-temperature.txt",light);
                                if temperature != 0 {
                                    let humidity = map_search("./temperature-humidity.txt",temperature);
                                    if humidity != 0 {
                                        let location = map_search("./humidity-location.txt",humidity);
                                        if location != 0 {
                                            if closest == 0 || location < closest{
                                                closest = location;
                                                answer = seed;
                                            }
                                        } else {
                                            println!("Unable to find location for {}", seed);
                                        }
                                    } else {
                                        println!("Unable to find humidity for {}", seed);
                                    }
                                } else {
                                    println!("Unable to find temperature for {}", seed);
                                }
                            } else {
                                println!("Unable to find light for {}", seed);
                            }
                        } else {
                            println!("Unable to find water for {}", seed);
                        }
                    } else {
                        println!("Unable to find fertilizer for {}", seed);
                    }
                } else {
                    println!("Unable to find soil for {}", seed);
                }
            }
            println!("Closest seed: {}", answer);
        }
    }
}

fn map_search(filename: &str, input: u64) -> u64 {
    let mut res : u64 = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let pieces = line.split_whitespace().collect::<Vec<&str>>();
            let inp = pieces[0].parse::<u64>().unwrap();
            let outp = pieces[1].parse::<u64>().unwrap();
            let rang = pieces[2].parse::<u64>().unwrap();
            if (input >= inp) && (input <= (inp + rang)) {
                res = outp + (input-inp)
            }
        }
    }
    res
}

// File line iterator from rust docs
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
