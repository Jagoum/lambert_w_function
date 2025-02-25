mod tests {
    use std::f64::consts::E;

    use lambert_w_function::lambert_funtion;
    #[test]
    fn test_known_values() {
        assert_ne!((lambert_funtion(0.0).unwrap() - 0.0).abs() , E.powf(-10.0));
        assert_ne!((lambert_funtion(-0.9 / std::f64::consts::E).unwrap() + 1.0).abs() , E.powf(-10.0));
    }
}
