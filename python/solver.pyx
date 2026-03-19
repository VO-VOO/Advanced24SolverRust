# cython: language_level=3

from libc.math cimport fabs, sqrt, log, pow, isfinite

cdef double TARGET = 24.0
cdef double EPS = 1e-6
cdef double DIV_EPS = 1e-12
cdef double VALUE_BOUND = 1000000.0
cdef int MAX_FACTORIAL = 10
cdef int MAX_UNARY_DEPTH = 2


cdef inline bint _close(double a, double b):
    return fabs(a - b) <= EPS


cdef inline bint _valid_number(double value):
    return isfinite(value) and fabs(value) <= VALUE_BOUND


cdef inline long long _factorial_int(int value):
    cdef int i
    cdef long long result = 1
    for i in range(2, value + 1):
        result *= i
    return result


cdef inline int _bucket(double value):
    if value >= 0:
        return <int>(value * 1000000.0 + 0.5)
    return <int>(value * 1000000.0 - 0.5)


cdef list _append_unique(list states, double value, str expr, int unary_depth):
    cdef tuple item
    for item in states:
        if _close(item[0], value):
            return states
    states.append((value, expr, unary_depth))
    return states


cdef list _expand_unary(double value, str expr, int unary_depth):
    cdef list states = [(value, expr, unary_depth)]
    cdef int rounded
    cdef long long fact_value
    cdef double result

    if unary_depth >= MAX_UNARY_DEPTH:
        return states

    if value >= -EPS:
        result = sqrt(value if value > 0 else 0.0)
        if _valid_number(result):
            _append_unique(states, result, f"sqrt({expr})", unary_depth + 1)

    rounded = <int>(value + 0.5)
    if _close(value, rounded) and 0 <= rounded <= MAX_FACTORIAL:
        fact_value = _factorial_int(rounded)
        result = <double>fact_value
        if _valid_number(result):
            _append_unique(states, result, f"({expr})!", unary_depth + 1)

    return states


cdef bint _same_value_exists(list values, double candidate):
    cdef tuple item
    for item in values:
        if _close(item[0], candidate):
            return True
    return False


cdef list _combine_states(tuple left, tuple right):
    cdef double a = left[0]
    cdef double b = right[0]
    cdef str expr_a = left[1]
    cdef str expr_b = right[1]
    cdef list results = []
    cdef double result
    cdef double rounded_b
    cdef double rounded_a

    result = a + b
    if _valid_number(result):
        results.append((result, f"({expr_a} + {expr_b})", 0))

    result = a - b
    if _valid_number(result) and not _same_value_exists(results, result):
        results.append((result, f"({expr_a} - {expr_b})", 0))

    result = b - a
    if _valid_number(result) and not _same_value_exists(results, result):
        results.append((result, f"({expr_b} - {expr_a})", 0))

    result = a * b
    if _valid_number(result) and not _same_value_exists(results, result):
        results.append((result, f"({expr_a} * {expr_b})", 0))

    if fabs(b) > DIV_EPS:
        result = a / b
        if _valid_number(result) and not _same_value_exists(results, result):
            results.append((result, f"({expr_a} / {expr_b})", 0))

    if fabs(a) > DIV_EPS:
        result = b / a
        if _valid_number(result) and not _same_value_exists(results, result):
            results.append((result, f"({expr_b} / {expr_a})", 0))

    if not (a < 0 and not _close(b, <int>(b + 0.5))) and fabs(b) <= 10.0:
        result = pow(a, b)
        if _valid_number(result) and not _same_value_exists(results, result):
            results.append((result, f"({expr_a} ^ {expr_b})", 0))

    if not (b < 0 and not _close(a, <int>(a + 0.5))) and fabs(a) <= 10.0:
        result = pow(b, a)
        if _valid_number(result) and not _same_value_exists(results, result):
            results.append((result, f"({expr_b} ^ {expr_a})", 0))

    if a > 0 and b > 0 and not _close(b, 1.0):
        result = log(a) / log(b)
        if _valid_number(result) and not _same_value_exists(results, result):
            results.append((result, f"log_{expr_b}({expr_a})", 0))

    if b > 0 and a > 0 and not _close(a, 1.0):
        result = log(b) / log(a)
        if _valid_number(result) and not _same_value_exists(results, result):
            results.append((result, f"log_{expr_a}({expr_b})", 0))

    return results


cdef tuple _canonical_key(list states):
    cdef list buckets = []
    cdef tuple item
    for item in states:
        buckets.append(_bucket(item[0]))
    buckets.sort()
    return tuple(buckets)


cdef str _dfs(list states, set visited):
    cdef int n = len(states)
    cdef int i
    cdef int j
    cdef int k
    cdef tuple state
    cdef tuple left_variant
    cdef tuple right_variant
    cdef tuple key
    cdef list remaining
    cdef list left_variants
    cdef list right_variants
    cdef list combined_states
    cdef list result_variants
    cdef str answer

    key = _canonical_key(states)
    if key in visited:
        return None
    visited.add(key)

    if n == 1:
        state = states[0]
        if _close(state[0], TARGET):
            return state[1]
        return None

    for i in range(n):
        for j in range(i + 1, n):
            remaining = []
            for k in range(n):
                if k != i and k != j:
                    remaining.append(states[k])

            left_variants = _expand_unary(states[i][0], states[i][1], states[i][2])
            right_variants = _expand_unary(states[j][0], states[j][1], states[j][2])

            for left_variant in left_variants:
                for right_variant in right_variants:
                    combined_states = _combine_states(left_variant, right_variant)
                    for state in combined_states:
                        result_variants = _expand_unary(state[0], state[1], state[2])
                        for state in result_variants:
                            answer = _dfs(remaining + [state], visited)
                            if answer is not None:
                                return answer
    return None


def solve_cards(tuple numbers):
    cdef list states = []
    cdef int number

    for number in numbers:
        states.append((<double>number, str(number), 0))

    return _dfs(states, set())
