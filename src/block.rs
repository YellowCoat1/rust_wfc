

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Block {
    Empty,
    UpperT,
    RightT,
    DownT,
    LeftT
}

pub const BLOCKS: [Block; 5] = [Block::Empty, Block::UpperT, Block::RightT, Block::DownT, Block::LeftT];

#[derive(Debug)]
pub struct BlockSides{
    pub up: bool, 
    pub right: bool, 
    pub down: bool, 
    pub left: bool,
}

pub fn get_block_sides(b: Block) -> BlockSides {
    // rotated because the array itself is tilted
    match b {
        Block::Empty => BlockSides{up: false, right: false, down: false, left: false},
        Block::UpperT => BlockSides{up: true, right: true, down: false, left: true},
        Block::RightT => BlockSides{up: true, right: true, down: true, left: false},
        Block::DownT => BlockSides{up: false, right: true, down: true, left: true},
        Block::LeftT => BlockSides{up: true, right: false, down: true, left: true}, 
    }
}

pub struct BlockArray{
    pub array: Vec<Vec<Option<Block>>>,
    pub width: usize,
    pub length: usize,
}

impl BlockArray {
    pub fn new_array(width: usize, length: usize) -> BlockArray {
        let top_vec = vec![vec![None; width]; length];
        BlockArray {
            array: top_vec,
            width,
            length,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Block> {
        if x > self.width - 1 || y > self.length - 1 {
            return None
        }

        self.array[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, val: Option<Block>) {
        if x > self.width - 1 || y > self.length - 1 {
            panic!("Out of bounds block array set")
        }

        self.array[y][x] = val;
    }

    // pub fn is_all_set(&self) -> bool {
    //     self.array.iter()
    //         .flatten()
    //         .find(|b| b.is_none())
    //         .is_none()
    // }
    
    pub fn iter(&self) -> impl Iterator<Item = &Vec<Option<Block>>> {
        self.array.iter()
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = (usize, usize, Option<Block>)> + '_ {
        self.iter()
            .enumerate()
            .flat_map(|(x, v)| v.iter().cloned().enumerate().map(move |(y, v)| (y, x, v) ))
    }

    pub fn add_row(&mut self) {
        self.length += 1;
        self.array.push(vec![None; self.width]);
    }

}