pub fn levenshtein_dist(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    s1.chars().enumerate().for_each(|(i, c1)| {
        s2.chars().enumerate().for_each(|(j, c2)| {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                matrix[i][j + 1] + 1,
                std::cmp::min(matrix[i + 1][j] + 1, matrix[i][j] + cost),
            );
        });
    });

    return matrix[len1][len2];
}

pub fn levenshtein_dist_normalized(s1: &str, s2: &str) -> f64 {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let dist = levenshtein_dist(s1, s2) as f64;
    let max_len = std::cmp::max(len1, len2) as f64;
    return dist / max_len;
}

pub fn levenshtein_similarity(s1: &str, s2: &str) -> f64 {
    return 1.0 / (1.0 + levenshtein_dist_normalized(s1, s2));
}
