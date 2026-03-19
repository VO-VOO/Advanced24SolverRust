use std::collections::HashSet;

pub const TARGET: f64 = 24.0;
pub const EPS: f64 = 1e-6;
const DIV_EPS: f64 = 1e-12;
const VALUE_BOUND: f64 = 1_000_000.0;
const MAX_FACTORIAL: u32 = 10;
const MAX_UNARY_DEPTH: u8 = 2;
const EXPONENT_BOUND: f64 = 10.0;

#[derive(Clone, Debug)]
struct State {
    value: f64,
    expr: String,
    unary_depth: u8,
}

pub fn solve_cards(numbers: [u8; 4]) -> Option<String> {
    let states = numbers
        .into_iter()
        .map(|number| State { value: f64::from(number), expr: number.to_string(), unary_depth: 0 })
        .collect::<Vec<_>>();

    dfs(states, &mut HashSet::new())
}

fn dfs(states: Vec<State>, visited: &mut HashSet<Vec<i64>>) -> Option<String> {
    let key = canonical_key(&states);
    if visited.contains(&key) {
        return None;
    }
    visited.insert(key);

    if states.len() == 1 {
        let state = &states[0];
        return close(state.value, TARGET).then(|| state.expr.clone());
    }

    for left_index in 0..states.len() {
        for right_index in (left_index + 1)..states.len() {
            let remaining = states
                .iter()
                .enumerate()
                .filter(|(index, _)| *index != left_index && *index != right_index)
                .map(|(_, state)| state.clone())
                .collect::<Vec<_>>();

            let left_variants = expand_unary(&states[left_index]);
            let right_variants = expand_unary(&states[right_index]);

            for left in &left_variants {
                for right in &right_variants {
                    for combined in combine_states(left, right) {
                        for variant in expand_unary(&combined) {
                            let mut next_states = remaining.clone();
                            next_states.push(variant);
                            if let Some(answer) = dfs(next_states, visited) {
                                return Some(answer);
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

fn canonical_key(states: &[State]) -> Vec<i64> {
    let mut buckets = states.iter().map(|state| bucket(state.value)).collect::<Vec<_>>();
    buckets.sort_unstable();
    buckets
}

fn combine_states(left: &State, right: &State) -> Vec<State> {
    let mut results = Vec::with_capacity(10);

    push_unique(
        &mut results,
        left.value + right.value,
        format!("({} + {})", left.expr, right.expr),
    );
    push_unique(
        &mut results,
        left.value - right.value,
        format!("({} - {})", left.expr, right.expr),
    );
    push_unique(
        &mut results,
        right.value - left.value,
        format!("({} - {})", right.expr, left.expr),
    );
    push_unique(
        &mut results,
        left.value * right.value,
        format!("({} * {})", left.expr, right.expr),
    );

    if right.value.abs() > DIV_EPS {
        push_unique(
            &mut results,
            left.value / right.value,
            format!("({} / {})", left.expr, right.expr),
        );
    }

    if left.value.abs() > DIV_EPS {
        push_unique(
            &mut results,
            right.value / left.value,
            format!("({} / {})", right.expr, left.expr),
        );
    }

    if valid_power(left.value, right.value) {
        push_unique(
            &mut results,
            left.value.powf(right.value),
            format!("({} ^ {})", left.expr, right.expr),
        );
    }

    if valid_power(right.value, left.value) {
        push_unique(
            &mut results,
            right.value.powf(left.value),
            format!("({} ^ {})", right.expr, left.expr),
        );
    }

    if valid_log(left.value, right.value) {
        push_unique(
            &mut results,
            left.value.ln() / right.value.ln(),
            format!("log_{}({})", right.expr, left.expr),
        );
    }

    if valid_log(right.value, left.value) {
        push_unique(
            &mut results,
            right.value.ln() / left.value.ln(),
            format!("log_{}({})", left.expr, right.expr),
        );
    }

    results
}

fn expand_unary(state: &State) -> Vec<State> {
    let mut states = vec![state.clone()];
    if state.unary_depth >= MAX_UNARY_DEPTH {
        return states;
    }

    if state.value >= -EPS {
        let result = if state.value > 0.0 { state.value.sqrt() } else { 0.0 };
        push_unique_with_depth(
            &mut states,
            result,
            format!("sqrt({})", state.expr),
            state.unary_depth + 1,
        );
    }

    if let Some(integer) = approx_integer(state.value)
        && integer <= MAX_FACTORIAL
    {
        push_unique_with_depth(
            &mut states,
            factorial(integer) as f64,
            format!("({})!", state.expr),
            state.unary_depth + 1,
        );
    }

    states
}

fn push_unique(states: &mut Vec<State>, value: f64, expr: String) {
    push_unique_with_depth(states, value, expr, 0);
}

fn push_unique_with_depth(states: &mut Vec<State>, value: f64, expr: String, unary_depth: u8) {
    if !valid_number(value) || states.iter().any(|state| close(state.value, value)) {
        return;
    }

    states.push(State { value, expr, unary_depth });
}

fn valid_number(value: f64) -> bool {
    value.is_finite() && value.abs() <= VALUE_BOUND
}

fn valid_power(base: f64, exponent: f64) -> bool {
    if exponent.abs() > EXPONENT_BOUND {
        return false;
    }

    if base < 0.0 && approx_integer(exponent).is_none() {
        return false;
    }

    let result = base.powf(exponent);
    valid_number(result)
}

fn valid_log(argument: f64, base: f64) -> bool {
    argument > 0.0 && base > 0.0 && !close(base, 1.0)
}

fn approx_integer(value: f64) -> Option<u32> {
    if value < -EPS {
        return None;
    }

    let rounded = value.round();
    if close(value, rounded) && rounded >= 0.0 { Some(rounded as u32) } else { None }
}

fn factorial(value: u32) -> u64 {
    (2..=value).fold(1_u64, |accumulator, item| accumulator * u64::from(item))
}

fn bucket(value: f64) -> i64 {
    if value >= 0.0 {
        (value * 1_000_000.0 + 0.5) as i64
    } else {
        (value * 1_000_000.0 - 0.5) as i64
    }
}

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= EPS
}

#[cfg(test)]
mod tests {
    use super::{EPS, solve_cards};

    #[test]
    fn solves_basic_hand() {
        let solution = solve_cards([1, 1, 7, 7]);
        assert!(solution.is_some());
    }

    #[test]
    fn solves_high_cards() {
        let solution = solve_cards([10, 11, 12, 13]);
        assert!(solution.is_some());
    }

    #[test]
    fn tolerance_constant_matches_spec() {
        assert!((EPS - 1e-6).abs() < f64::EPSILON);
    }
}
