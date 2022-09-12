pub fn elo(k: f64, rwinner: usize, rloser: usize) -> (usize, usize) {
    let qw = f64::powf(10.0, rwinner as f64 / 400.0);
    let ql = f64::powf(10.0, rloser as f64 / 400.0);
    let el = ql / (qw + ql);
    (rwinner + (k * el) as usize, rloser - (k * el) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_elo() {
        assert_eq!(elo(200.0, 1000, 1000), (1100, 900));
        assert_eq!(elo(200.0, 1200, 800), (1218, 782));
        assert_eq!(elo(200.0, 800, 1200), (981, 1019));
    }
}
