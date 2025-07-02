/*
[1]
[1,1]
[1,2,1]
[1,3,3,1]

<10m
 */
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut p: Vec<Vec<i32>> = vec![vec![1], vec![1,1]];
    if num_rows <= p.len() as i32 {
        return p[0..num_rows as usize].to_vec();
    }

    for i in 2..num_rows as usize {
        let prev = &p[i-1];
        let mut cur = vec![1; prev.len() + 1];

        for j in 0..cur.len() {
            let a = prev.get(j-1);
            let b = prev.get(j);

            if let (Some(a), Some(b)) = (a, b) {
                cur[j] = a + b;
            }
        }

        p.push(cur);

    }

    p[0..num_rows as usize].to_vec()
}