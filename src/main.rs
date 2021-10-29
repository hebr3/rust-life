extern crate rand;

const WIDTH: usize = 5;
const HEIGHT: usize = 4;

type World = [[u8; WIDTH]; HEIGHT];

fn main() {
    let mut world: World = [[0u8; WIDTH]; HEIGHT];
    let mut generations = 0;
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if rand::random() {
                world[i][j] = 1;
            } else {
                world[i][j] = 0;
            }
        }
    }
    let mut census_count = census(world);
    while census_count > 0 && generations < 10 {
        census_count = census(world);
        print_world(world);
        println!("generation {} population count count: {}", generations, census_count);
        world = generation(world);
        generations += 1;
    }
}

fn census(_world: World) -> u16 {
    let mut count: u16 = 0;

    for row in _world {
        for val in row {
            count += u16::from(val);
        }
    }
    count
}

fn generation(world: World) -> World {
    let mut newworld = [[0u8; WIDTH]; HEIGHT];
 
    for i in 0..HEIGHT-1 {
        for j in 0..WIDTH-1 {
            let mut count = 0;
            if i>0 {
                count += world[i-1][j];
            }
            if i>0 && j>0 {
                count += world[i-1][j-1];
            }
            if i>0 && j<74 {
                count += world[i-1][j+1];
            }
            if i<74 && j>0 {
                count += world[i+1][j-1]
            }
            if i<74 {
                count += world[i+1][j];
            }
            if i<74 && j<74 {
                count += world[i+1][j+1];
            }
            if j>0 {
                count += world[i][j-1];
            }
            if j<74 {
                count += world[i][j+1];
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

fn print_world(_world: World) {
    for y in _world {
        println!("{:?}", y);
    }
}