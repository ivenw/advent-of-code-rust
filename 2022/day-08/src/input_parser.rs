use ndarray::Array2;

pub fn parse_input(input: &str) -> Array2<u32> {
    let trees = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    Array2::from_shape_vec((trees.len(), trees[0].len()), trees.concat()).unwrap()
}
