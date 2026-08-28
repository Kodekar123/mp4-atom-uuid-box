use crate::*;

pub struct C2paBox {
    pub box_purpouse: String,
}

impl AtomExt for C2paBox {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() -> Result<()> {
        Ok(())
    }
}
