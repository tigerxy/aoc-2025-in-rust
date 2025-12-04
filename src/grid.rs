use std::ops::{Index, IndexMut};

const OFFSETS_4: &[(isize, isize)] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];
const OFFSETS_8: &[(isize, isize)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Debug, Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn from_rows(rows: Vec<Vec<T>>) -> Self {
        let height = rows.len();
        assert!(height > 0, "grid must have at least one row");

        let width = rows[0].len();
        assert!(width > 0, "grid must have at least one column");
        assert!(
            rows.iter().all(|r| r.len() == width),
            "all rows must have the same length"
        );

        let data = rows.into_iter().flatten().collect();
        Self::new(width, height, data)
    }

    pub fn new(width: usize, height: usize, data: Vec<T>) -> Self {
        assert_eq!(width * height, data.len());
        Self {
            width,
            height,
            data,
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[inline]
    fn index_of(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(self.idx(x, y))
        } else {
            None
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.index_of(x, y).map(|i| &self.data[i])
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.index_of(x, y).map(|i| &mut self.data[i])
    }

    /// All coordinates of the grid.
    pub fn coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| (x, y)))
    }

    /// Iterate over all cells as `Cell` views.
    pub fn all(&self) -> impl Iterator<Item = Cell<'_, T>> + '_ {
        self.coords().map(move |(x, y)| Cell {
            x,
            y,
            value: &self[(x, y)],
            grid: self,
        })
    }

    #[inline]
    fn neighbors_with<'a>(
        &'a self,
        x: usize,
        y: usize,
        offsets: &'static [(isize, isize)],
    ) -> impl Iterator<Item = ((usize, usize), &'a T)> + 'a {
        offsets.iter().filter_map(move |(dx, dy)| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && nx < self.width as isize && ny < self.height as isize {
                let (ux, uy) = (nx as usize, ny as usize);
                Some(((ux, uy), &self[(ux, uy)]))
            } else {
                None
            }
        })
    }

    /// 4-neighbors of a coordinate (top, right, bottom, left).
    pub fn neighbors_4(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = ((usize, usize), &T)> + '_ {
        self.neighbors_with(x, y, OFFSETS_4)
    }

    /// 8-neighbors of a coordinate (including diagonals).
    pub fn neighbors_8(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = ((usize, usize), &T)> + '_ {
        self.neighbors_with(x, y, OFFSETS_8)
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[self.idx(x, y)]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let idx = self.idx(x, y);
        &mut self.data[idx]
    }
}

/// A view on a single cell of the grid, with handy neighbor methods.
#[derive(Clone, Copy, Debug)]
pub struct Cell<'g, T> {
    pub x: usize,
    pub y: usize,
    pub value: &'g T,
    grid: &'g Grid<T>,
}

impl<'g, T> Cell<'g, T> {
    #[inline]
    fn neighbors_with(
        self,
        offsets: &'static [(isize, isize)],
    ) -> impl Iterator<Item = Cell<'g, T>> + 'g {
        self.grid
            .neighbors_with(self.x, self.y, offsets)
            .map(move |((x, y), value)| Cell {
                x,
                y,
                value,
                grid: self.grid,
            })
    }

    pub fn neighbors_4(self) -> impl Iterator<Item = Cell<'g, T>> + 'g {
        self.neighbors_with(OFFSETS_4)
    }

    pub fn neighbors_8(self) -> impl Iterator<Item = Cell<'g, T>> + 'g {
        self.neighbors_with(OFFSETS_8)
    }
}
