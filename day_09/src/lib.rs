use std::collections::HashSet;

use head_move::HeadMove;

pub mod head_move;

pub fn calculate_unique_position_count(moves: &[HeadMove], knot_count: usize) -> usize {
    if knot_count <= 0 {
        panic!("Knot count has to be larger than 0");
    }

    let mut knots = (0..knot_count).map(|_| (0, 0)).collect::<Vec<(i64, i64)>>();
    let mut tail_positions = HashSet::new();
    tail_positions.insert((0, 0));

    for head_move in moves {
        match head_move {
            HeadMove::Up(offset) => for _ in 0..*offset {
                knots[0] = (knots[0].0, knots[0].1 + 1);
                for i in 1..knot_count {
                    knots[i] = calculate_new_tail_position(&knots[i - 1], &knots[i]);
                }
                tail_positions.insert(knots.last().unwrap().clone());
            },
            HeadMove::Right(offset) => for _ in 0..*offset {
                knots[0] = (knots[0].0 + 1, knots[0].1);
                for i in 1..knot_count {
                    knots[i] = calculate_new_tail_position(&knots[i - 1], &knots[i]);
                }
                tail_positions.insert(knots.last().unwrap().clone());
            },
            HeadMove::Down(offset) => for _ in 0..*offset {
                knots[0] = (knots[0].0, knots[0].1 - 1);
                for i in 1..knot_count {
                    knots[i] = calculate_new_tail_position(&knots[i - 1], &knots[i]);
                }
                tail_positions.insert(knots.last().unwrap().clone());
            },
            HeadMove::Left(offset) => for _ in 0..*offset {
                knots[0] = (knots[0].0 - 1, knots[0].1);
                for i in 1..knot_count {
                    knots[i] =  calculate_new_tail_position(&knots[i - 1], &knots[i]);
                }
                tail_positions.insert(knots.last().unwrap().clone());
            }
        }
    }
    tail_positions.len()
}

fn calculate_new_tail_position(head_start_position: &(i64, i64), tail_start_position: &(i64, i64)) -> (i64, i64) {
    if head_start_position == tail_start_position {
        return tail_start_position.clone();
    }

    let head_x = head_start_position.0;
    let head_y = head_start_position.1;
    
    let tail_x = tail_start_position.0;
    let tail_y = tail_start_position.1;

    if head_x == tail_x {
        let new_tail_y = if head_y > tail_y + 1 { 
            tail_y + 1 
        } else if head_y < tail_y - 1 {
            tail_y - 1
        } else {
            tail_y
        };
        
        return (tail_x, new_tail_y);
    }

    if head_y == tail_y {
        let new_tail_x = if head_x > tail_x + 1 {
            tail_x + 1
        } else if head_x < tail_x - 1 {
            tail_x - 1
        } else {
            tail_x
        };

        return (new_tail_x, tail_y);
    }
    
    if head_y > tail_y + 1 {
        if head_x > tail_x {
            return (tail_x + 1, tail_y + 1);
        } else {
            return (tail_x - 1, tail_y + 1);
        }
    }

    if head_y < tail_y - 1 {
        if head_x > tail_x {
            return (tail_x + 1, tail_y - 1);
        } else {
            return (tail_x - 1, tail_y - 1);
        }
    }

    if head_x > tail_x + 1 {
        if head_y > tail_y {
            return (tail_x + 1, tail_y + 1);
        } else {
            return (tail_x + 1, tail_y - 1);
        }
    }

    if head_x < tail_x - 1 {
        if head_y > tail_y {
            return (tail_x - 1, tail_y + 1);
        } else {
            return (tail_x - 1, tail_y - 1);
        }
    }
    
    tail_start_position.clone()
}
