use std::io::{stdin, Read};

const WIDTH: u32 = 25;
const HEIGHT: u32 = 6;

fn main() {
    let mut image_string = String::new();
    stdin().read_to_string(&mut image_string).unwrap();

    let image: Vec<u32> = image_string.trim().chars().map(|pix| pix.to_digit(10).unwrap()).collect();
    let num_layers = image.len() as u32 / (WIDTH * HEIGHT);

    let mut layers: Vec<Vec<u32>> = Vec::new();  

    let mut fewest_zeros: (u32, usize) = (u32::max_value(), 0);

    let mut iterator = image.iter();
    for i in 0..num_layers {
        layers.push(Vec::new());

        let mut num_zeros = 0;
        for _x in 0..WIDTH {
            for _y in 0..HEIGHT {
                let digit = *iterator.next().unwrap();
                if digit == 0 {
                    num_zeros += 1;
                }

                layers[i as usize].push(digit);
            }
        }

        if num_zeros < fewest_zeros.0 {
            fewest_zeros = (num_zeros, i as usize);
        }
    }

    let layer = &layers[fewest_zeros.1];

    let ones: usize = layer.clone().into_iter().filter(|&digit| digit == 1).count();
    let twos: usize = layer.clone().into_iter().filter(|&digit| digit == 2).count();

    println!("{}", ones * twos);
}
