pub struct Triangle {
    side1: u64,
    side2: u64,
    side3: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let less_than_zero_sides = sides.iter().any(|x| *x <= 0);

        let mut sorted_sides = sides;
        sorted_sides.sort();
        let sum_of_sides = sorted_sides[2] <= (sorted_sides[0] + sorted_sides[1]);

        if sides.is_empty() || less_than_zero_sides || !sum_of_sides {
            return None;
        } else {
            return Some(Triangle{side1: sides[0], side2: sides[1], side3: sides[2]});
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.side1 == self.side2 && self.side1 == self.side3
    }

    pub fn is_scalene(&self) -> bool {
        self.side1 != self.side2 && self.side2 != self.side3 && self.side1 != self.side3
    }

    pub fn is_isosceles(&self) -> bool {
        self.side1 == self.side2 || self.side2 == self.side3 || self.side1 == self.side3
    }
}


