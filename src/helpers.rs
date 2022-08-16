/**
A multidimensional version of Bresenham's algorithm.  It's used to convert weights into indexes, used by [MutationIter] and [CrossoverIter]

It'd be really nice if this function was const, and it theoretically could be, but Vec currently can't be used in const functions and it'd be hard to refactor to not use Vec.
*/
pub fn multidimensional_bresenhams(weights: &[usize]) -> Vec<usize> {
    let mut result: Vec<usize> = vec![];
    if weights.len() == 0 {
        return result;
    }
    let max = *weights.iter().max().unwrap() as isize;
    let mut errs: Vec<isize> = vec![0; weights.len()];
    for _ in 0..max {
        for i in 0..weights.len() {
            if errs[i] >= 0 {
                result.push(i);
                errs[i] -= max;
            }
            errs[i] += weights[i] as isize;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::multidimensional_bresenhams;

    #[test]
    fn test_bres() {
        assert_eq!(
            &multidimensional_bresenhams(&[2usize, 3, 1]),
            &[0, 1, 2, 1, 0, 1]
        );
    }
}
