use std::collections::{HashSet, LinkedList};

pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let mut visits = vec![i32::MIN; is_connected.len()];
    let mut q: LinkedList<(i32, i32)> = LinkedList::from_iter((0..is_connected.len() as i32).zip(0..is_connected.len() as i32));

    // From each starting point, traverse as far as you can go marking the starting point
    while let Some((from, city)) = q.pop_front() {
        if visits[city as usize] != i32::MIN {
            continue;
        }

        visits[city as usize] = from;

        for j in 0..is_connected[city as usize].len() {
            if (j as i32 == city) { continue; }
            if (is_connected[city as usize][j] == 1) {
                q.push_front((from, j as i32))
            }
        }
    }

    // Each unique starting point is a set of connected components
    HashSet::<i32>::from_iter(visits.iter().cloned()).len() as i32
}
