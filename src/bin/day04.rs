use std::{env, iter, str::FromStr};

use aoc2025::{get_file_name, read_lines};

fn main() {
    let args: Vec<String> = env::args().collect();
    let infile = get_file_name(&args).unwrap();

    let lines = read_lines(infile);
    let tiles: Vec<Vec<FloorTile>> = lines.iter().map(|s| s.chars().map(|c| c.to_string().parse().unwrap()).collect()).collect();
    let mut grid = PaddedGrid::from_rows(tiles, FloorTile::Empty);

    let mut accessible = 0;
    grid.for_each_with_neighbors(
        |c, neighbors| {
            match c {
                FloorTile::Empty => (),
                FloorTile::Paper => {
                    let count = neighbors.iter().map(|n|
                        {
                            match n {
                            FloorTile::Empty => 0,
                            FloorTile::Paper => 1,
                        }}
                    ).reduce(|acc, e| acc + e).unwrap();
                    if count < 4 {
                        accessible += 1;
                    }
                }
            }
        }
    );

    println!("Accessible: {accessible}");

    let mut paper_count = paper_in_grid(&grid);
    let starting_count = paper_count;
    println!("Starting with {paper_count} paper");

    let mut new_count: u32 = 0;
    while new_count != paper_count {
        paper_count = new_count;
        grid = grid.map_with_neighbors(FloorTile::Empty, |t, neighbors| {
            match t {
                FloorTile::Empty => FloorTile::Empty,
                FloorTile::Paper => {
                    let count = neighbors.iter().map(|n| {
                        match n {
                            FloorTile::Empty => 0,
                            FloorTile::Paper => 1,
                        }
                    }).reduce(|acc, e| acc + e).unwrap();
                    if count < 4 {
                        return FloorTile::Empty
                    }
                    return FloorTile::Paper
                }
            }
        });

        new_count = paper_in_grid(&grid);
    }

    let removed = starting_count - new_count;
    println!("Ending with {new_count}");
    println!("We removed: {removed}");


}

fn paper_in_grid(grid: &PaddedGrid<FloorTile>) -> u32 {
    let mut res = 0;
    grid.for_each_with_neighbors(|f, _| { if *f == FloorTile::Paper {res += 1}});
    res
}


#[derive(Debug,Clone,PartialEq)]
enum FloorTile {
    Empty,
    Paper,
}

#[derive(Debug)]
enum ParseFloorTileError {
    BadCharacter,
    EmptyString,
}

impl FromStr for FloorTile {
    type Err = ParseFloorTileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars().next()
            .ok_or(ParseFloorTileError::EmptyString)
            .and_then(|c| match c {
                '.' => Ok(FloorTile::Empty),
                '@' => Ok(FloorTile::Paper),
                _ => Err(ParseFloorTileError::BadCharacter),
            })
    }
}

struct PaddedGrid<T: Clone> {
    width: usize,
    _height: usize, // Turned out to not really be needed
    elements: Vec<T>,
}

impl <T: Clone> PaddedGrid<T> {
    pub fn from_rows<I, R>(rows: I, default: T) -> PaddedGrid<T> 
    where
        I: IntoIterator<Item = R>,
        R: IntoIterator<Item = T>,
    {
        let mut unpadded: Vec<T> = vec![];
        let mut height: usize = 0;
        let mut width: usize = 0;
        for row in rows {
            height += 1;
            let mut row_width: usize = 0;
            for e in row {
                unpadded.push(e);
                row_width += 1;
            }
            if width == 0 {
                width = row_width;
            } else if width != row_width {
                panic!("Mismatched widths in PaddedGrid input!");
            }
        }

        PaddedGrid {
            width,
            _height: height,
            elements: pad_flat_grid(unpadded, width, default)
        }
    }

    pub fn for_each_with_neighbors<F>(&self, mut f: F)
    where F: FnMut(&T, [&T; 8])
    {
        let padded_width = self.width + 2;
        let rows: Vec<_> = self.elements.chunks(padded_width).collect();

        for window in rows.windows(3) {
            let (top, middle, bottom) = (&window[0], &window[1], &window[2]);
            let top_windows = top.windows(3);
            let middle_windows = middle.windows(3);
            let bottom_windows = bottom.windows(3);

            for square in top_windows.zip(middle_windows).zip(bottom_windows) {
                let ((top_window, middle_window), bottom_window) = square;
                let cell = &middle_window[1];
                let neighbors = [
                    &top_window[0], &top_window[1], &top_window[2],
                    &middle_window[0], &middle_window[2],
                    &bottom_window[0], &bottom_window[1], &bottom_window[2],
                ];

                f(cell, neighbors);
            }
        }
    }

    pub fn map_with_neighbors<F>(&self, default: T, mut f: F) -> PaddedGrid<T>
    where F: FnMut(&T, [&T; 8]) -> T
    {
        let mut new_elements = Vec::with_capacity(self.width * self._height);

        self.for_each_with_neighbors(|cell, neighbors| {
            new_elements.push(f(cell, neighbors));
        });

        PaddedGrid {
            width: self.width,
            _height: self._height,
            elements: pad_flat_grid(new_elements, self.width, default) 
        }
    }
}

fn pad_flat_grid<T>(elements: Vec<T>, width: usize, default: T) -> Vec<T>
where T: Clone
{
    iter::repeat(default.clone())
        .take(width+2)
        .chain(
            elements.chunks(width).flat_map(|row| {
                iter::once(default.clone()).chain(
                    row.iter().cloned()
                ).chain(iter::once(default.clone()))
            })
        )
        .chain(iter::repeat(default.clone()).take(width+2))
        .collect::<Vec<T>>()
}