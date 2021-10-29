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
 
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let cell = world[i][j];
            let nw = world[(i+HEIGHT-1) % HEIGHT][(j+WIDTH-1) % WIDTH];
            let n = world[(i+HEIGHT-1) % HEIGHT][(j+WIDTH) % WIDTH];
            let ne = world[(i+HEIGHT-1) % HEIGHT][(j+WIDTH+1) % WIDTH];
            let e = world[(i+HEIGHT) % HEIGHT][(j+WIDTH+1) % WIDTH];
            let se = world[(i+HEIGHT+1) % HEIGHT][(j+WIDTH+1) % WIDTH];
            let s = world[(i+HEIGHT+1) % HEIGHT][(j+WIDTH) % WIDTH];
            let sw = world[(i+HEIGHT+1) % HEIGHT][(j+WIDTH-1) % WIDTH];
            let w = world[(i+HEIGHT) % HEIGHT][(j+WIDTH-1) % WIDTH];
 
            let count = nw + n + ne + e + se + s + sw + w;
            newworld[i][j] = 0;
 
            if (count <2) && (cell == 1) {
                newworld[i][j] = 0;
            }
            if cell == 1 && (count == 2 || count == 3) {
                newworld[i][j] = 1;
            }
            if (cell == 0) && (count == 3) {
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
