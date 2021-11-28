use anyhow::Result;
use std::path::Path;

pub fn main(_path: &Path) -> Result<(usize, Option<usize>)> {
    Ok((0, None))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() -> Result<()> {
        Ok(())
    }
}
