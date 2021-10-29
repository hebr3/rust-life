extern crate rand;
use std::{thread, time};
use rand::Rng;

fn main() {
    let mut world = [[0u8; 75]; 75];
    let mut generations = 0;
    for i in 0..74 {
        for j in 0..74 {
            if rand::random() {
                world[i][j] = 1;
            } else {
                world[i][j] = 0;
            }
        }
    }
    for i in 0..100 {
        let c = census(world);
        println!("generation {} population count count: {}", generations, c);
        world = generation(world);
        generations += 1;
    }
}

fn census(_world: [[u8; 75]; 75]) -> u16 {
    let mut count = 0;

    for i in 0..74 {
        for j in 0..74 {
            if _world[i][j] == 1 {
                count += 1;
            }
        }
    }
    count
}

fn generation(world: [[u8; 75]; 75]) -> [[u8; 75]; 75] {
    let mut newworld = [[0u8; 75]; 75];
 
    for i in 0..74 {
        for j in 0..74 {
            let mut count = 0;
            if i>0 {
                count = count + world[i-1][j];
            }
            if i>0 && j>0 {
                count = count + world[i-1][j-1];
            }
            if i>0 && j<74 {
                count = count + world[i-1][j+1];
            }
            if i<74 && j>0 {
                count = count + world[i+1][j-1]
            }
            if i<74 {
                count = count + world[i+1][j];
            }
            if i<74 && j<74 {
                count = count + world[i+1][j+1];
            }
            if j>0 {
                count = count + world[i][j-1];
            }
            if j<74 {
                count = count + world[i][j+1];
            }
 
            newworld[i][j] = 0;
 
            if (count <2) && (world[i][j] == 1) {
                newworld[i][j] = 0;
            }
            if world[i][j] == 1 && (count == 2 || count == 3) {
                newworld[i][j] = 1;
            }
            if (world[i][j] == 0) && (count == 3) {
                newworld[i][j] = 1;
            }
        }
    }
    newworld
}