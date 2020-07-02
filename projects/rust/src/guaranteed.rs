fn guaranteed_hard(p_raw: impl Into<f32>, max_raw: impl Into<i32>) -> f32 {
    let p = p_raw.into();
    let max = max_raw.into();
    assert!(p >= 0.0, "");
    assert!(p <= 1.0, "");
    assert!(p >= 0.0, "");
    assert!(max >= 0, "");
    // End of standardization
    return 0.0;
}