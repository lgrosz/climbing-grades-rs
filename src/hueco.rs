pub struct Grade {
    value: u8,
}

impl Grade {
    pub fn new(value: u8) -> Grade {
        Grade { value }
    }

    pub fn value(& self) -> u8 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(Grade::new(1).value(), 1);
    }
}

