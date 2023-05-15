#[derive(Debug, Clone, Default)]
struct Image<P> {
    pixels: Vec<P>,
    width: usize,
    height: usize,
}

impl<P: Default + Copy> Image<P> {
    fn new(width: usize, height: usize) -> Self {
        Image {
            pixels: vec![P::default(); width * height],
            width,
            height,
        }
    }
}
// Returns a slice of the pixels in the row
// so we can access the individual pixels in the row
impl<P> std::ops::Index<usize> for Image<P> {
    type Output = [P];
    fn index(&self, row: usize) -> &[P] {
        let start = row * self.width;
        &self.pixels[start..start + self.width]
    }
}

// Returns a slice of the pixels in the row
// so we can access the individual pixels in the row
impl<P> std::ops::IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, row: usize) -> &mut [P] {
        let start = row * self.width;
        &mut self.pixels[start..start + self.width]
    }
}

fn main() {
    let mut image = Image::new(10, 10);
    image[0][0] = 1;
    image[0][1] = 2;
    image[0][2] = 3;
    image[0][3] = 4;
    image[0][4] = 5;
    image[0][5] = 6;
    image[0][6] = 7;
    image[0][7] = 8;
    image[0][8] = 9;
    image[0][9] = 10;
    let row: usize = 0;
    let col: usize = 10;
    println!("row: {}, val: {:?}", row, &image[row]);
    println!("row: {}, col: {}, val: {:?}", row, col, &image[row][col]);
    let row: usize = 2;
    println!("row: {}, col: {}, val: {:?}", row, col, &image[row][col]);
}
