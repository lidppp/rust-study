#[derive(Debug)]
struct Rectangles {
    width: f64,
    height: f64,
}
/**
 * 可以有多个impl 关联同一个结构体
 */
impl Rectangles {
    fn area(&self) -> f64 {
        println!("rect: {:#?}", self);
        dbg!(&self);
        self.width * self.height
    }

    fn from_Rectangles(rect: &Rectangles) -> Rectangles {
        Rectangles {
            width: rect.width,
            height: rect.height,
        }
    }

    fn from_w_h(width: f64, height: f64) -> Rectangles {
        Rectangles { width, height }
    }

    fn add(&mut self, rect: &Rectangles) {
        self.width += rect.width;
        self.height += rect.height;
    }
}

fn main() {
    let rect = Rectangles {
        width: 12.1f64,
        height: 23.5f64,
    };
    let area_by_rect = rect.area();
    println!("area: {:?}", area_by_rect);
    println!("area: {:.2}", 0.1 + 0.2);
}
