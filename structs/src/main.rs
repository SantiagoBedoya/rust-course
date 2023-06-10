#![allow(dead_code)]

struct Size(usize, usize);

impl Size {
    const S720P: Self = Size(1280, 720);
    const S1080P: Self = Size(1920, 1080);

    fn new(width: usize, height: usize) -> Self {
        assert!(width > 0 && height > 0);
        Self(width, height)
    }
}

struct Image {
    pixels: Vec<u8>,
    size: Size,
}

impl Image {
    fn new(size: Size) -> Self {
        Self{
            pixels: vec![0; size.0 * size.1],
            size,
        }
    }
    
    fn to_string(&self) -> String {
        format!("Image: {}x{} - {} pixels", self.size.0, self.size.1, self.pixels.len())
    }

    fn reset(&mut self, size: Size) {
        self.pixels = vec![0; size.0 * size.1];
        self.size = size;
    }

    fn into_parts(self) -> (Vec<u8>, Size) {
        (self.pixels, self.size)
    }
}

fn main() {
    

    let img_1 = Image::new(Size::new(0, 0));
    println!("{}", img_1.to_string());

    let mut img_2 = Image::new(Size::S1080P);
    println!("{}", img_2.to_string());

    img_2.reset(Size::S720P);
    println!("{}", img_2.to_string());

    let (pixels, size) = img_2.into_parts();
    println!("{} {} {}", pixels.len(), size.0, size.1);
    
}
