
mod tests {
    use lambert_w_function::lambert_funtion;
    #[test]
    fn test_known_values() {
        assert!((lambert_funtion(0.0).unwrap() - 0.0).abs() < 1e-10);
        assert!((lambert_funtion(-1.0 / std::f64::consts::E).unwrap() + 1.0).abs() < 1e-10);
    }
}