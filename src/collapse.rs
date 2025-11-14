use crate::block::BLOCKS;

use super::block;

#[derive(Debug)]
struct Sides {
    up: Option<bool>,
    right: Option<bool>,
    down: Option<bool>,
    left: Option<bool>,
}

pub fn collapse_array(array: &mut block::BlockArray) {

    loop {

        #[cfg(feature = "debug")]
        {
            super::print_block::print_block_array(array, owo_colors::Style::new(), owo_colors::Style::new());
            let _ = std::io::stdin().read_line(&mut String::new());
        }

        // gets least entropy block
        let (block_x, block_y, mut possibilities) = match find_least_entropy(array) {
            Some(s) => s,
            // if there is none, break
            None  => break
        };

        #[cfg(feature = "debug")]
        println!("x: {} y: {} possibilities: {:?}", block_x, block_y, possibilities);


        // in case there are none availible, choose just any
        if possibilities.len() == 0 {possibilities = BLOCKS.to_vec()};
        
        let possibility_selected_index = rand::random::<u8>() as usize % possibilities.len();
        let possibility_selected = possibilities[possibility_selected_index];

        array.set(block_x, block_y, Some(possibility_selected));

    }
}

pub fn inverse_collapse_array(array: &mut block::BlockArray) {
    loop {
        let (block_x, block_y, possibilities) = match find_least_entropy(array) {
            Some(s) => s,
            None => break,
        };

        let mut inverse_possibilities: Vec<_> = BLOCKS.iter()
            .filter(|v| !possibilities.contains(v))
            .copied()
            .collect();

        // in case the array is empty
        if inverse_possibilities.len() == 0 {inverse_possibilities = BLOCKS.to_vec()};


        let possibility_selected_index = rand::random::<u8>() as usize % inverse_possibilities.len();
        let possibility_selected = inverse_possibilities[possibility_selected_index];

        array.set(block_x, block_y, Some(possibility_selected))
    }
}


fn find_least_entropy(array: &block::BlockArray) -> Option<(usize, usize, Vec<block::Block>)> {

    let mut least_entropy = vec![];
    let mut smallest_entropy = None;

    for (x, y, block_type) in array.iter_coords() {
        
        if let Some(_) = block_type {
            continue
        }

        let allowed_sides = find_allowed_sides(array, x, y);
        let possibilites = find_possibilities_from_sides(allowed_sides);

        let entropy = possibilites.len();
        
        if let Some(n) = smallest_entropy {
            if entropy < n {
                smallest_entropy = Some(entropy);
                least_entropy.clear();
                least_entropy.push((x, y, possibilites));
            } else if entropy == n {
                least_entropy.push((x, y, possibilites));
            }
        } else {
            smallest_entropy = Some(entropy);
            least_entropy.push((x, y, possibilites));
        }

    }

    if least_entropy.len() == 0 {
        return None
    }


    let random_block_index = rand::random::<u8>() as usize % least_entropy.len();
    
    Some(least_entropy[random_block_index].clone())
}

fn find_allowed_sides(array: &block::BlockArray, x: usize, y: usize) -> Sides {
    macro_rules! array_get_sides {
        ($x_pos: expr, $y_pos: expr) => {
            array.get($x_pos, $y_pos).map(|b| block::get_block_sides(b))
        };
    }   



    let up = if y > 0 {
        array_get_sides!(x, y-1)
            .map(|s| s.down)
    } else {
        None
    };
    
    let right = array_get_sides!(x+1, y)
        .map(|s| s.left);

    let down = array_get_sides!(x, y+1)
        .map(|s| s.up);

    let left = if x > 0 {
        array_get_sides!(x-1, y)
            .map(|s| s.right)
    } else {
        None
    };

    // println!("Left {:?} {:?}", left, array_get_sides!(x, y-1));

    // println!("{:?} {:?} {:?} {:?}", up, down, left, right);
    
    Sides {
        up,
        down,
        left,
        right
    }
}

fn find_possibilities_from_sides(sides: Sides) -> Vec<block::Block> {
    let mut allowed_blocks = vec![];

    for block_type in block::BLOCKS.iter() {
        let block_type_sides = block::get_block_sides(*block_type);

        if let Some(s) = sides.up {
            if block_type_sides.up != s {continue}
        }

        if let Some(s) = sides.right {
            if block_type_sides.right != s {continue}
        }

        if let Some(s) = sides.down {
            if block_type_sides.down != s {continue}
        }

        if let Some(s) = sides.left {
            if block_type_sides.left != s {continue}
        }

        allowed_blocks.push(*block_type);
    }

    allowed_blocks
}