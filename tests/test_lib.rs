use rustyfresh::{mean, variance, skewness, extract_features};

#[test]
fn test_mean() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(mean(data), 3.0);
}

#[test]
fn test_variance() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let var = variance(data);
    assert!((var - 2.5).abs() < 1e-6);
}

#[test]
fn test_skewness() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let skew = skewness(data);
    assert!((skew - 0.0).abs() < 1e-6);
}

#[test]
fn test_extract_features() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let features = extract_features(data);
    assert_eq!(features.get("mean").unwrap(), &3.0);
    assert!((features.get("variance").unwrap() - 2.5).abs() < 1e-6);
    assert!((features.get("skewness").unwrap() - 0.0).abs() < 1e-6);
}
