use std::collections::{HashSet};
use rand::{Rng, thread_rng};
use std::fmt;
use std::fmt::{Display, Formatter};
use petgraph::algo::connected_components;
use petgraph::graphmap::UnGraphMap;
use rand::rngs::ThreadRng;

const ROW_LENGTH: u8 = 6;
const MAX_BARRIERS: u8 = 24;

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    VeryHard,
}

impl Difficulty {
    pub fn barrier_num(&self) -> u8 {
        match *self {
            Difficulty::Easy => 12,
            Difficulty::Medium => 16,
            Difficulty::Hard => 20,
            Difficulty::VeryHard => MAX_BARRIERS,
        }
    }
}

pub struct BarrierNum {
    value: u8,
}

impl BarrierNum {
    pub fn new(value: u8) -> Option<Self> {
        if value > MAX_BARRIERS {
            return None;
        }

        Some(
            BarrierNum {
                value
            })
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

pub struct Board {
    graph: UnGraphMap<u8, u8>,
    // Edges that can be removed while keeping the graph connected
    can_rem_edge_set: Vec<(u8, u8)>,
    barrier_edge_set: HashSet<(u8, u8)>,
    rng: ThreadRng,
}

impl Board {
    fn new() -> Self {
        let can_rem_edge_set = Self::new_edge_set();
        let graph = UnGraphMap::from_edges(&can_rem_edge_set);

        Board {
            graph,
            can_rem_edge_set,
            barrier_edge_set: HashSet::new(),
            rng: thread_rng(),
        }
    }

    fn new_edge_set() -> Vec<(u8, u8)> {
        let mut edge_set = Vec::new();
        let limit = ROW_LENGTH - 1;

        for i in 0..ROW_LENGTH {
            for j in 0..ROW_LENGTH {
                if j < limit {
                    let x = i * ROW_LENGTH + j;
                    edge_set.push((x, x + 1));
                }

                if i < limit {
                    let x = ROW_LENGTH * i + j;
                    edge_set.push((x, x + ROW_LENGTH));
                }
            }
        }

        edge_set
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `barrier_num`: The number of barriers wanted
    ///
    /// returns: Board
    ///
    /// # Examples
    ///
    /// ```
    /// use labyrinth_maker::{BarrierNum, Board};
    /// let board = Board::build_board_barnum(BarrierNum::new(13).unwrap());
    /// ```
    pub fn build_board_barnum(barrier_num: BarrierNum) -> Self {
        let mut board = Self::new();

        let barrier_num = barrier_num.value() as usize;
        while board.barrier_edge_set.len() < barrier_num {
            board.remove_rand_edge();
        }

        board
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `difficulty`: The difficulty wanted for the board
    ///
    /// returns: Board
    ///
    /// # Examples
    ///
    /// ```
    /// use labyrinth_maker::{Board, Difficulty};
    /// let board = Board::build_board_dif(Difficulty::VeryHard);
    /// ```
    pub fn build_board_dif(difficulty: Difficulty) -> Self {
        let mut board = Self::new();

        let barrier_num = difficulty.barrier_num() as usize;
        while board.barrier_edge_set.len() < barrier_num {
            board.remove_rand_edge();
        }

        board
    }

    fn remove_rand_edge(&mut self) {
        let mut remove_random_edge = |graph: &mut UnGraphMap<u8, u8>| {
            let index = self.rng.gen_range(0..self.can_rem_edge_set.len());
            let x = self.can_rem_edge_set.swap_remove(index);

            graph.remove_edge(x.0, x.1);

            x
        };

        let mut edge = remove_random_edge(&mut self.graph);
        while connected_components(&self.graph) != 1 {
            self.graph.add_edge(edge.0, edge.1, 0);

            edge = remove_random_edge(&mut self.graph);
        }

        self.barrier_edge_set.insert(edge);
    }
}

const OC_TOP_BARRIER: &str = "---";
const FR_TOP_BARRIER: &str = "- -";
const OC_LEFT_BARRIER: &str = "|";
const FR_LEFT_BARRIER: &str = "¦";
impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let border = OC_TOP_BARRIER.repeat(8) + "-";
        f.write_str(&border)?;
        f.write_str("\n")?;

        let limit = ROW_LENGTH - 1;
        for i in 0..ROW_LENGTH {
            f.write_str(OC_LEFT_BARRIER)?;
            f.write_str("\t")?;
            for j in 0..ROW_LENGTH {
                if j < limit {
                    let x = ROW_LENGTH * i + j;
                    if self.barrier_edge_set.contains(&(x, x + 1)) {
                        f.write_str(OC_LEFT_BARRIER)?;
                    } else {
                        f.write_str(FR_LEFT_BARRIER)?;
                    }
                    f.write_str("\t")?;
                } else {
                    f.write_str(OC_LEFT_BARRIER)?;
                }
            }
            f.write_str("\n")?;

            if i < limit {
                for j in 0..ROW_LENGTH {
                    let x = i * ROW_LENGTH + j;
                    f.write_str(" ")?;
                    if self.barrier_edge_set.contains(&(x, x + ROW_LENGTH)) {
                        f.write_str(OC_TOP_BARRIER)?;
                    } else {
                        f.write_str(FR_TOP_BARRIER)?;
                    }
                }
                f.write_str("\n")?;
            }
        }

        f.write_str(&border)?;

        Ok(())
    }
}