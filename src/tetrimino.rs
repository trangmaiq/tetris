extern crate rand;

// We have two ways of handling the tetrimino rotation:
// using matrix rotation or storing the different states.
// To have a code that easy to read and update, I picked the second option,
// but it'd nice to try using matrix later, it could help to learn a lot of things.
struct Tetrimino {
    states: Vec<Vec<Vec<u8>>>,
    x: isize,
    y: isize,
    current_state: u8,
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
