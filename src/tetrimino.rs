extern crate rand;

// We have two ways of handling the tetrimino rotation:
// using matrix rotation or storing the different states.
// To have a code that easy to read and update, I picked the second option,
// but it'd nice to try using matrix later, it could help to learn a lot of things.
struct Tetrimino {
    states: Vec<Vec<Vec<u8>>>,
    x: isize,
    y: usize,
    current_state: u8,
}

impl Tetrimino {
    fn rotate(&mut self, game_map: &[Vec<u8>]) {
        // A bit longer, indeed. Since we can't be sure that
        // the piece will be put where we want it to go, we need to make temporary variables
        // and then check the possibilities. We use the temporary variables before going further.
        let mut tmp_state = self.current_state + 1;
        if tmp_state as usize >= self.states.len() {
            tmp_state = 0;
        }
        // This line its own doesn't make much sense but it'll be very useful next:
        // in case the piece cannot be placed where we want, we try to move it on the `x` axis
        // to see if it'd work in some other place. It allows you to have a Tetris
        // that is much more flexible and comfortable to play
        let x_pos = [0, -1, 1, -2, 2, -3];
        // With the explanations given previously, this loop should be really easy to understand.
        // For each x shift, we check whether the piece can be placed there.
        // If it works, we change the values of our tetrimino, otherwise we just continue.
        // If no `x` shift worked, we just leave the function without doing anything.
        for x in x_pos.iter() {
            if self.test_position(game_map, tmp_state as usize, self.x + x, self.y) == true {
                self.current_state = tmp_state;
                self.x += *x;
                break
            }
        }
    }

    // Now that we can rotate and test the position of a tetrimino,
    // it'd be nice to actually move it as well
    // (when the timer hoes to 0 and the tetrimino needs to go down, for example)
    // If the tetrimino cannot move, we'll return a Boolean value
    // to allow the caller to be aware of it.
    // We don't check possible positions, just the one received. The reason is simple,
    // contrary to a rotation, we can't move the tetrimino around when it receives a move instruction.
    // Imaging asking the tetrimino to move to the right and it doesn't move,
    // or worse, it moves to the left.
    // We can't allow it and so we're not doing it.
    //
    // If we can put the tetrimino in a place, we update the position of the tetrimino and return true,
    // otherwise, we do nothing other than return false.
    fn change_position(&mut self, game_map: &[Vec<u8>], new_x: isize, new_y: usize) -> bool {
        if self.test_position(game_map, self.current_state as usize, new_x, new_y) == true {
            self.x = new_x as isize;
            self.y = new_y;
            true
        } else {
            false
        }
    }

    // It seems important to explain why the game map became a &[Vec<u8>].
    // When we send a non-mutable reference over a vector (Vec<T>),
    // it is then dereferenced into a & &[T] slice,
    // which is a constant `view` over the vector's content.
    //
    // We loop over every block of our `tetrimino`
    // and check whether the block is free in the game map
    // (by checking whether it is equal to 0) and if it isn't going out of the game map.
    fn test_position(&self, game_map: &[Vec<u8>], tmp_state: usize, x: isize, y: usize) -> bool {
        for decal_y in 0..4 {
            for decal_x in 0..4 {
                let x = x + decal_x;
                if self.states[tmp_state][decal_y][decal_x as usize] != 0
                    && (y + decal_y >= game_map.len()
                    || x < 0
                    || x as usize >= game_map[y + decal_y].len()
                    ||game_map[y + decal_y][x as usize] != 0
                ) {
                    return false;
                }
            }
        }
        return true;
    }
}

trait TetriminoGenerator {
    fn new() -> Tetrimino;
}

struct TetriminoI;

struct TetriminoJ;

struct TetriminoL;

struct TetriminoO;

struct TetriminoS;

struct TetriminoZ;

struct TetriminoT;

impl TetriminoGenerator for TetriminoI {
    fn new() -> Tetrimino {
        Tetrimino {
            // In here, a number represents a color and zero means no color
            // (because there's no block).
            states: vec![vec![vec![1, 1, 1, 1],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
            ],
                         vec![vec![1, 0, 0, 0],
                              vec![1, 0, 0, 0],
                              vec![1, 0, 0, 0],
                              vec![1, 0, 0, 0],
                         ]
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

impl TetriminoGenerator for TetriminoJ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![2, 2, 2, 0],
                              vec![0, 0, 2, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
            ],
                         vec![vec![0, 2, 0, 0],
                              vec![0, 2, 0, 0],
                              vec![2, 2, 0, 0],
                              vec![0, 0, 0, 0],
                         ],
                         vec![vec![2, 0, 0, 0],
                              vec![2, 2, 2, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
                         ],
                         vec![vec![2, 2, 0, 0],
                              vec![2, 0, 0, 0],
                              vec![2, 0, 0, 0],
                              vec![0, 0, 0, 0],
                         ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

impl TetriminoGenerator for TetriminoL {
    fn new() -> Tetrimino {
        Tetrimino {
            // The answer for: Why the blokcs have `2` as value,
            // it's simple so that we can differentiate them when displaying
            // (having all tetrimino with the same color wouldn't be very pretty).
            // It has no other meaning.
            states: vec![vec![vec![3, 3, 3, 0],
                              vec![3, 0, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
            ],
                         vec![vec![3, 3, 0, 0],
                              vec![0, 3, 0, 0],
                              vec![0, 3, 0, 0],
                              vec![0, 0, 0, 0],
                         ],
                         vec![vec![0, 0, 3, 0],
                              vec![3, 3, 3, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
                         ],
                         vec![vec![3, 0, 0, 0],
                              vec![3, 0, 0, 0],
                              vec![3, 3, 0, 0],
                              vec![0, 0, 0, 0],
                         ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

impl TetriminoGenerator for TetriminoO {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![4, 4, 0, 0],
                              vec![4, 4, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
            ]],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

impl TetriminoGenerator for TetriminoS {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![0, 5, 5, 0],
                              vec![5, 5, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
            ],
                         vec![vec![0, 5, 0, 0],
                              vec![0, 5, 5, 0],
                              vec![0, 0, 5, 0],
                              vec![0, 0, 0, 0],
                         ]
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

impl TetriminoGenerator for TetriminoZ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![6, 6, 0, 0],
                              vec![0, 6, 6, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
            ],
                         vec![vec![0, 0, 6, 0],
                              vec![0, 6, 6, 0],
                              vec![0, 6, 0, 0],
                              vec![0, 0, 0, 0],
                         ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

impl TetriminoGenerator for TetriminoT {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![7, 7, 7, 0],
                              vec![0, 7, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
            ],
                         vec![vec![0, 7, 0, 0],
                              vec![7, 7, 0, 0],
                              vec![0, 7, 0, 0],
                              vec![0, 0, 0, 0],
                         ],
                         vec![vec![0, 7, 0, 0],
                              vec![7, 7, 7, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
                         ],
                         vec![vec![0, 7, 0, 0],
                              vec![0, 7, 7, 0],
                              vec![0, 7, 0, 0],
                              vec![0, 0, 0, 0],
                         ]
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

fn create_new_tetrimino() -> Tetrimino {
    // If we just call rand::random(), this is a bit too random.
    // It's d be problematic if we had the same tetrimino generated more than twice in a row.
    // (which is already a lot!), so we need to improve it by adding a static variable.
    // A bit explanation might be helpful here.
    // First, what is a `static` variable?
    // That is a variable that will keep its value
    // and won't be destroyed when the scope it has been created inside has been left.
    static mut PREV: u8 = 7;
    let mut rand_nb = rand::random::<u8>() % 7;

    if unsafe {PREV} == rand_nb {
        rand_nb = rand::random::<u8>() % 7;
    }

    unsafe {PREV = rand_nb; }

    match rand_nb {
        0 => TetriminoI::new(),
        1 => TetriminoJ::new(),
        2 => TetriminoL::new(),
        3 => TetriminoO::new(),
        4 => TetriminoS::new(),
        5 => TetriminoZ::new(),
        6 => TetriminoT::new(),
        _ => unreachable!(),
    }
}
