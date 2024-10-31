#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }

    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width > another.width && self.height > another.height
    }

    fn square(size: usize) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let rect = Rectangle {
            width: 10,
            height: 20,
        };
        assert_eq!(rect.area(), 200);
    }

    #[test]
    fn test_area_zero_width() {
        let rect = Rectangle {
            width: 0,
            height: 20,
        };
        assert_eq!(rect.area(), 0);
    }

    #[test]
    fn test_area_zero_height() {
        let rect = Rectangle {
            width: 10,
            height: 0,
        };
        assert_eq!(rect.area(), 0);
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rectangle is {:?}", rectangle.area());
    println!("Can rect1 hold rect2? {}", rectangle.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rectangle.can_hold(&rect3));
}
