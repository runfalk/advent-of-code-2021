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

#[test]
fn test_day12() -> Result<()> {
    assert_eq!(
        run_day(12, advent_of_code_2021::day12::main)?,
        (4912, Some(150004))
    );
    Ok(())
}

#[test]
fn test_day13() -> Result<()> {
    let mut b = String::new();
    b.push_str(" ##  ###  #  # #### ###   ##  #  # #  #\n");
    b.push_str("#  # #  # #  #    # #  # #  # #  # #  #\n");
    b.push_str("#  # #  # ####   #  #  # #    #  # ####\n");
    b.push_str("#### ###  #  #  #   ###  #    #  # #  #\n");
    b.push_str("#  # # #  #  # #    #    #  # #  # #  #\n");
    b.push_str("#  # #  # #  # #### #     ##   ##  #  #\n");

    assert_eq!(
        run_day(13, advent_of_code_2021::day13::main)?,
        (747, Some(b))
    );
    Ok(())
}

#[test]
fn test_day14() -> Result<()> {
    assert_eq!(
        run_day(14, advent_of_code_2021::day14::main)?,
        (2851, Some(10002813279337))
    );
    Ok(())
}

#[test]
fn test_day15() -> Result<()> {
    assert_eq!(
        run_day(15, advent_of_code_2021::day15::main)?,
        (390, Some(2814))
    );
    Ok(())
}

#[test]
fn test_day16() -> Result<()> {
    assert_eq!(
        run_day(16, advent_of_code_2021::day16::main)?,
        (879, Some(539051801941))
    );
    Ok(())
}

#[test]
fn test_day17() -> Result<()> {
    assert_eq!(
        run_day(17, advent_of_code_2021::day17::main)?,
        (2628, Some(1334))
    );
    Ok(())
}

#[test]
fn test_day19() -> Result<()> {
    assert_eq!(
        run_day(19, advent_of_code_2021::day19::main)?,
        (398, Some(10965))
    );
    Ok(())
}
