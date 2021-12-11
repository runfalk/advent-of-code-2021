use anyhow::Result;
use std::path::Path;

fn run_day<A, B>(day: usize, f: fn(&Path) -> Result<(A, Option<B>)>) -> Result<(A, Option<B>)> {
    f(format!("data/day{}.txt", day).as_ref())
}

#[test]
fn test_day1() -> Result<()> {
    assert_eq!(
        run_day(1, advent_of_code_2021::day1::main)?,
        (1676, Some(1706))
    );
    Ok(())
}

#[test]
fn test_day2() -> Result<()> {
    assert_eq!(
        run_day(2, advent_of_code_2021::day2::main)?,
        (1488669, Some(1176514794))
    );
    Ok(())
}

#[test]
fn test_day3() -> Result<()> {
    assert_eq!(
        run_day(3, advent_of_code_2021::day3::main)?,
        (3958484, Some(1613181))
    );
    Ok(())
}

#[test]
fn test_day5() -> Result<()> {
    assert_eq!(
        run_day(5, advent_of_code_2021::day5::main)?,
        (6572, Some(21466))
    );
    Ok(())
}

#[test]
fn test_day6() -> Result<()> {
    assert_eq!(
        run_day(6, advent_of_code_2021::day6::main)?,
        (362666, Some(1640526601595))
    );
    Ok(())
}

#[test]
fn test_day7() -> Result<()> {
    assert_eq!(
        run_day(7, advent_of_code_2021::day7::main)?,
        (349812, Some(99763899))
    );
    Ok(())
}

#[test]
fn test_day8() -> Result<()> {
    assert_eq!(
        run_day(8, advent_of_code_2021::day8::main)?,
        (525, Some(1083859))
    );
    Ok(())
}

#[test]
fn test_day9() -> Result<()> {
    assert_eq!(
        run_day(9, advent_of_code_2021::day9::main)?,
        (577, Some(1069200))
    );
    Ok(())
}

#[test]
fn test_day10() -> Result<()> {
    assert_eq!(
        run_day(10, advent_of_code_2021::day10::main)?,
        (392421, Some(2769449099))
    );
    Ok(())
}

#[test]
fn test_day11() -> Result<()> {
    assert_eq!(
        run_day(11, advent_of_code_2021::day11::main)?,
        (1694, Some(346))
    );
    Ok(())
}
