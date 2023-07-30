#[cfg(test)]
mod tests {
    #[test]
    fn test_dive () {
        assert!(true)
    }
}

pub mod depths {
    pub const DIVE_DEPTHS: [u8; 24] = [ 10,  15,  20,  25,  30,  35,  40,  45,
                                        50,  55,  60,  70,  80,  90, 100, 110,
                                       120, 130, 140, 150, 160, 170, 180, 190];
}
