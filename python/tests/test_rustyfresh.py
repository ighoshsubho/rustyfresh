import rustyfresh

def test_mean():
    data = [1.0, 2.0, 3.0, 4.0, 5.0]
    assert rustyfresh.mean(data) == 3.0

def test_variance():
    data = [1.0, 2.0, 3.0, 4.0, 5.0]
    var = rustyfresh.variance(data)
    assert abs(var - 2.5) < 1e-6

def test_skewness():
    data = [1.0, 2.0, 3.0, 4.0, 5.0]
    skew = rustyfresh.skewness(data)
    assert abs(skew - 0.0) < 1e-6

def test_extract_features():
    data = [1.0, 2.0, 3.0, 4.0, 5.0]
    features = rustyfresh.extract_features(data)
    assert features["mean"] == 3.0
    assert abs(features["variance"] - 2.5) < 1e-6
    assert abs(features["skewness"] - 0.0) < 1e-6
    
if __name__ == "__main__":
    test_mean()
    test_variance()
    test_skewness()
    test_extract_features()
    print("All tests passed.")
