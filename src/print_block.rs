use super::block;
use owo_colors::{OwoColorize, Style};
use owo_colors::colors::{CustomColor, White};
type Pink = CustomColor<245, 171, 185>;
type Blue = CustomColor<91, 207, 251>;

type PrintArray = Vec<Vec<Option<bool>>>;

pub fn print_block_array(block_array: &block::BlockArray, fill_gaps: bool ,set_style: owo_colors::Style, empty_style: owo_colors::Style, trans_colors: bool) {
    let print_arr_width = block_array.width * 3;
    let print_arr_length = block_array.length * 3;

    let mut print_fragments: PrintArray = vec![vec![None; print_arr_width]; print_arr_length];

    for (block_x, row) in block_array.iter().enumerate() {
        for (block_y, block_type) in row.iter().enumerate() {
            if let Some(s) = block_type {
                add_block_to_print_array(&mut print_fragments, *s, block_x, block_y);
            }
        }
    }  

    print_print_array(&print_fragments, fill_gaps, set_style, empty_style, trans_colors);
}

pub fn print_last_row(block_array: &block::BlockArray, fill_gaps: bool, set_style: owo_colors::Style, empty_style: owo_colors::Style, trans_enable: bool) {
    let print_arr_width = block_array.width * 3;
    let print_arr_length = 3;

    let mut print_fragments: PrintArray = vec![vec![None; print_arr_width]; print_arr_length];


    for (block_y, block_type) in block_array.iter().last().unwrap().iter().enumerate() {
        if let Some(s) = block_type {
            add_block_to_print_array(&mut print_fragments, *s, 0, block_y);
        }
    }

    let mut set_style = set_style;

    if trans_enable {
        let selected_color = ((block_array.array.len()-1) % 12).div_euclid(3);
        set_style = match selected_color {
            0 => Style::new().bg::<Blue>().fg::<Blue>(),
            1 => Style::new().bg::<Pink>().fg::<Pink>(),
            2 => Style::new().white().bg::<White>(),
            3 => Style::new().bg::<Pink>().fg::<Pink>(),
            other => panic!("{}", other)
        };
    }

    print_print_array(&print_fragments, fill_gaps, set_style, empty_style, false);
}

fn print_print_array(print_array: &PrintArray, print_imbetween: bool, set_style: owo_colors::Style, empty_style: owo_colors::Style, trans_colors: bool) {

    macro_rules! print_styled {
        ($text: expr, $style: expr) => {
            print!("{}", $text.style(*$style))
        };
    }

    let white = Style::new().white().bg::<White>();
    let blue = Style::new().bg::<Blue>().fg::<Blue>();
    let pink = Style::new().bg::<Pink>().fg::<Pink>();


    // let char_limit = 

    for (print_y, row) in print_array.iter().enumerate() {
        let mut set_style = &set_style;
        if trans_colors {
            let color_ind = (print_y % 12).div_euclid(3);
            set_style = match color_ind {
                0 => &blue,
                1 => &pink,
                2 => &white,
                3 => &pink,
                _ => panic!()
            }
        }
        for (print_x, val) in row.iter().enumerate() {


            if print_imbetween {
                let to_the_left = if print_x > 0 {
                    print_array[print_y][print_x-1]
                } else {
                    None
                };
    
                if let (Some(true), Some(true)) = (to_the_left, val) {
                    print_styled!(" ", set_style);
                } else {
                    print_styled!(" ", &empty_style);
                }
            } else {
                print_styled!(" ", &empty_style);
            }
    
            match val {
                Some(true) => print_styled!("#", set_style),
                Some(false) => print_styled!(" ", &empty_style),
                None => print!(" "),
            }

            if print_imbetween {
                let to_the_right = if print_array[print_y].len()-1 > print_x {
                    print_array[print_y][print_x+1]
                } else {
                    None
                };

                if let (Some(true), Some(true)) = (val, to_the_right) {
                    print_styled!(" ", set_style);
                }  else {
                    print_styled!(" ", &empty_style);
                }
            } else {
                print_styled!(" ", &empty_style);
            }
            
        }
        print_styled!("\n", &empty_style);
    }
}

fn add_block_to_print_array(print_array: &mut Vec<Vec<Option<bool>>>, block_type: block::Block, 
x: usize, y: usize) {
    let mut block_print = [[false; 3]; 3];


    match block_type {
        block::Block::Empty => (),
        _ => block_print[1][1] = true,
    }

    let block_sides = block::get_block_sides(block_type);
    block_print[1][0] = block_sides.up;
    block_print[2][1] = block_sides.right;
    block_print[1][2] = block_sides.down;
    block_print[0][1] = block_sides.left;

    // get iter of block_print coords
    // e.g. (0, 1, false)
    let print_coords = block_print
        .iter()
        .enumerate()
        .flat_map(|(x, v)| v.iter().enumerate().map(move |(y, v)| (y, x, v)));


    let block_offset_x = x*3;
    let block_offset_y = y*3;

    for (x, y, set) in print_coords {
        print_array[x+block_offset_x][y+block_offset_y] = Some(*set);
    }



}