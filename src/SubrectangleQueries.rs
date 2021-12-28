pub struct SubRectangleQueries {
    width: i32,
    matrix: Vec<i32>,
}

impl SubRectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        SubRectangleQueries {
            width: rectangle[0].len() as i32,
            matrix: rectangle.into_iter().flatten().collect::<Vec<i32>>(),
        }
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        for x in row1..=row2 {
            for y in col1..=col2 {
                let idx = self.xy_idx(x, y);
                self.matrix[idx] = new_value;
            }
        }
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        let idx = self.xy_idx(row, col);
        self.matrix[idx]
    }

    fn xy_idx(&self, x: i32, y: i32) -> usize {
        (x as usize * self.width as usize) + y as usize
    }
}

#[cfg(test)]
mod sub_rectangle_queries_tests {
    use super::*;

    pub enum Instructions {
        GetValue((Vec<i32>, i32)),
        UpdateSubRectangle(Vec<i32>),
    }

    fn assert_query_instructions(
        instructions: Vec<Instructions>,
        mut rect_queries: SubRectangleQueries,
    ) {
        use self::Instructions::*;

        for ins in instructions {
            match ins {
                GetValue((pos, expected)) => {
                    let result = rect_queries.get_value(pos[0], pos[1]);
                    assert_eq!(result, expected)
                }
                UpdateSubRectangle(pos) => {
                    let row1 = pos[0];
                    let col1 = pos[1];
                    let row2 = pos[2];
                    let col2 = pos[3];
                    let new_value = pos[4];

                    rect_queries.update_subrectangle(row1, col1, row2, col2, new_value);
                }
            }
        }
    }

    #[test]
    fn sub_rectangle_queries_one() {
        use self::Instructions::*;

        // arrange
        let tests = vec![
            GetValue((vec![0, 2], 1)),
            UpdateSubRectangle(vec![0, 0, 3, 2, 5]),
            GetValue((vec![0, 2], 5)),
            GetValue((vec![3, 1], 5)),
            UpdateSubRectangle(vec![3, 0, 3, 2, 10]),
            GetValue((vec![3, 1], 10)),
            GetValue((vec![0, 2], 5)),
        ];

        let rect_queries = SubRectangleQueries::new(vec![
            vec![1, 2, 1],
            vec![4, 3, 4],
            vec![3, 2, 1],
            vec![1, 1, 1],
        ]);

        assert_query_instructions(tests, rect_queries);
    }
}
