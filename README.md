# flow_control

Declarative macros for common control-flow use cases such as [`break`](https://doc.rust-lang.org/std/keyword.break.html), [`continue`](https://doc.rust-lang.org/std/keyword.continue.html), and [`return`](https://doc.rust-lang.org/std/keyword.return.html).

---

#### break_if

Break from a loop if a given predicate evaluates to true.

```rust
use flow_control::break_if;

break_if!(predicate);
break_if!(predicate, label);
```

---

#### continue_if

Continue to the next iteration of a loop if a given predicate evaluates to true.
```rust
use flow_control::continue_if;

continue_if!(predicate);
continue_if!(predicate, label);
```

---

#### return_if

Return from a function if a given predicate evaluates to true.
```rust
use flow_control::return_if;

return_if!(predicate);
return_if!(predicate, value);
```

## Examples

---

### break_if

#### Predicate only
```rust
use flow_control::break_if;

let mut v = Vec::new();
for outer_n in 1..3 {
    for inner_n in 1..5 {
        break_if!(inner_n == 3);
        v.push((outer_n, inner_n));
    }
}

assert_eq!(
    v,
    vec![
        (1, 1), (1, 2),
        (2, 1), (2, 2),
    ]
);
```

#### Predicate and label
```rust
use flow_control::break_if;

let mut v = Vec::new();
'outer: for outer_n in 1..3 {
    for inner_n in 1..5 {
        break_if!(inner_n == 3, 'outer);
        v.push((outer_n, inner_n));
    }
}

assert_eq!(
    v,
    vec![(1, 1), (1, 2)],
);
```

---

### continue_if

#### Predicate only
```rust
use flow_control::continue_if;

let mut v = Vec::new();
for outer_n in 1..3 {
    for inner_n in 1..5 {
        continue_if!(inner_n == 3);
        v.push((outer_n, inner_n));
    }
}

assert_eq!(
    v,
    vec![
        (1, 1), (1, 2), (1, 4),
        (2, 1), (2, 2), (2, 4),
    ]
);
```

#### Predicate and label
```rust
use flow_control::continue_if;

let mut v = Vec::new();
'outer: for outer_n in 1..3 {
    for inner_n in 1..5 {
        continue_if!(inner_n == 3, 'outer);
        v.push((outer_n, inner_n));
    }
}

assert_eq!(
    v,
    vec![
        (1, 1), (1, 2),
        (2, 1), (2, 2),
    ]
);
```

---

### return_if

#### Default return
```rust
use flow_control::return_if;

let mut v = Vec::new();
(|| {
    for n in 1..10 {
        return_if!(n == 5);
        v.push(n)
    }
})();

assert_eq!(v, vec![1, 2, 3, 4]);
```

#### Return a specified value
```rust
use flow_control::return_if;

let get_value = || {
    for n in 1..10 {
        return_if!(n == 5, "early return");
    }
    return "return after loop";
};

assert_eq!(get_value(), "early return");
```
