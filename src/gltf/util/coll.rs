pub fn flatten_optional_vector<T>(optional_vector: &Option<Vec<T>>) -> Vec<&T> {
    optional_vector.iter().flatten().collect()
}
