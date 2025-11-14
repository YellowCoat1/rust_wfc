mod block;
mod print_block;
mod collapse;
use termion;

use clap::Parser;

#[derive(Parser)]
#[command(name = "lynx_wfc")]
#[command(about = "A simple wave function collapse program written in rust")]
struct Conf {
    #[arg(short, long)]
    fill: bool,

    size: Option<usize>,

    #[arg(short, long)]
    width: Option<usize>,
    #[arg(short, long)]
    length: Option<usize>,


    // Use # instead of colored blocks
    #[arg(long)]
    bare: bool,

    // continually creates new rows and scrolls
    #[arg(short, long)]
    scroll: bool,

    // invervals inbetween generation for scrolling. Measured in Miliseconds. Default: 200
    #[arg(long)]
    interval: Option<u64>,

    // inverses the function, can only place what it usually cant
    #[arg(short, long)]
    inverse: bool,

    // TRANS
    #[arg(long)]
    trans: bool,
}


fn main() {
    let conf = Conf::parse();

    let (block_width, block_length) = get_array_size(&conf);

    let mut block_array: block::BlockArray = block::BlockArray::new_array(block_width, block_length);
    block_array.set(0, 0, Some(block::Block::RightT));
    let mut set_style = owo_colors::style().red();
    if !conf.bare {
        set_style = set_style.bg::<owo_colors::colors::Red>();
    }
    let empty_style = owo_colors::style().dimmed();
    
    if !conf.inverse {
        collapse::collapse_array(&mut block_array);
    } else {
        collapse::inverse_collapse_array(&mut block_array);
    }

    print_block::print_block_array(&block_array, true, set_style, empty_style, conf.trans);

    if !conf.scroll {
        return
    }

    let time_between = match conf.interval {
        Some(s) => s,
        None => 200,
    };

    loop {
        std::thread::sleep(std::time::Duration::from_millis(time_between));
        block_array.add_row();
        collapse::collapse_array(&mut block_array);
        print_block::print_last_row(&block_array, true, set_style, empty_style, conf.trans);
    }

}

fn get_array_size(conf: &Conf) -> (usize, usize) {
    let (mut length, mut width) = (None, None);
    if let Some(s) = conf.size {
        length = Some(s);
        width = Some(s);
    };

    if let Some(w) = conf.width {
        width = Some(w);
        length = length.or(Some(1));
    }
    if let Some(l) = conf.length {
        length = Some(l);
    }

    if let (Some(w), Some(l)) = (width, length) {
        return (w, l)
    } else {
        if let Ok((w, l)) = termion::terminal_size() {
            return ((w/9) as usize, (l/6) as usize);
        } else {
            eprintln!("Didn't provide an array size");
            std::process::exit(3);
        }
    }

}


// #[cfg(test)]
// mod tests {
//     use super::*
// }