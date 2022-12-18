use ndarray::Array2;

#[derive(Debug, Default, Clone)]
pub struct Tree {
    pub height: u32,
    pub h_visible: bool,
    pub v_visible: bool,
}

pub fn parse_input(input: &str) -> Array2<Tree> {
    let trees = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Tree {
                    height: c.to_digit(10).unwrap(),
                    ..Default::default()
                })
                .collect::<Vec<Tree>>()
        })
        .collect::<Vec<Vec<Tree>>>();

    Array2::from_shape_vec((trees.len(), trees[0].len()), trees.concat()).unwrap()
}
