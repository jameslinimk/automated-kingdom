use macroquad::rand::gen_range;

fn generate_shape(x: usize, y: usize) -> Vec<Vec<bool>> {
    let mut result = vec![vec![false; x]; y];

    // Get our starting position
    let start_x = gen_range(0, x);
    let start_y = gen_range(0, y);

    // Set our starting position
    result[start_y][start_x] = true;

    // Generate our shape, moving randomly in any direction
    let mut curr_x = start_x;
    let mut curr_y = start_y;
    let mut num_steps = 0;
    loop {
        num_steps += 1;
        match gen_range(0, 4) {
            0 => {
                // Up
                if curr_y > 0 {
                    result[curr_y - 1][curr_x] = true;
                    curr_y -= 1;
                }
            }
            1 => {
                // Down
                if curr_y < y - 1 {
                    result[curr_y + 1][curr_x] = true;
                    curr_y += 1;
                }
            }
            2 => {
                // Left
                if curr_x > 0 {
                    result[curr_y][curr_x - 1] = true;
                    curr_x -= 1;
                }
            }
            3 => {
                // Right
                if curr_x < x - 1 {
                    result[curr_y][curr_x + 1] = true;
                    curr_x += 1;
                }
            }
            _ => unreachable!(),
        }
        // If we have reached our starting position again, we have completed the shape.
        if curr_x == start_x && curr_y == start_y {
            // Make sure the shape isn't too small
            if num_steps as f32 >= (x * y) as f32 / 1.5 {
                break;
            }
        }
    }

    result
}

#[test]
fn test_generate_shape() {
    let size = 5;

    for _ in 0..5 {
        println!("New shape:");

        let shape = generate_shape(size, size);

        for row in shape.iter() {
            for cell in row.iter() {
                print!("{}", if *cell { "#" } else { " " });
            }
            println!();
        }
    }
}
