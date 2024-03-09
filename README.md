# `snowflake`

A `Twitter` `snowflake` algorithm simply implemented in `Rust`.

[Twitter Snowflake](https://github.com/twitter-archive/snowflake.git)

[APIs Documents](https://docs.rs/snowflaker)

## 1.`Usage`

Add this to your `Cargo.toml`:

```toml
[dependencies]
snowflaker = "0.3"

# Or
# @see SnowflakeGenerator::dynamic()
snowflaker = { version = "0.3", features = ["dynamic"] }
```

-- -

## 2.`APIs`

### 2.1.`Generator`

#### 2.1.1.`builtin`

```rust
// use the default data-center ID and worker ID
let gen = SnowflakeGenerator::builtin();
assert!(gen.is_ok());
```

#### 2.1.2.`new`

```rust
let gen = SnowflakeGenerator::new(31, 31);
assert!(gen.is_ok());

let gen = SnowflakeGenerator::new(32, 32);
assert!(gen.is_err());
```

#### 2.1.3.`dynamic`

- `@since 0.2.0`

```toml
[dependencies]
snowflaker = { version = "${version}", features = ["dynamic"] }
```

```rust
let gen = SnowflakeGenerator::dynamic();
assert!(gen.is_ok());

let rvt = gen.unwrap().next_id();
assert!(rvt.is_ok());
```

-- -

### 2.2.`Functions`

#### 2.2.1.`next_id`

```rust
// 122235238222008321
let rvt = snowflaker::next_id();
assert!(rvt.is_ok());
```

#### 2.2.2.`next_id_string`

```rust
// 122256588529602560
let rvt = snowflaker::next_id_string();
assert!(rvt.is_ok());
```

-- -

### 2.3.`Macros`

- `@since 0.3.0`

#### 2.3.1.`snowflake_builtin`

```rust
let rvt = snowflake_builtin!();
assert!(rvt.is_ok());
```

#### 2.3.2.`snowflake_builtin_string`

```rust
let rvt = snowflake_builtin_string!();
assert!(rvt.is_ok());
```

#### 2.3.3.`snowflake_dynamic`

```rust
let rvt = snowflake_dynamic!();
assert!(rvt.is_ok());
```

#### 2.3.4.`snowflake_dynamic_string`

```rust
let rvt = snowflake_dynamic_string!();
assert!(rvt.is_ok());
```

-- -

### 2.4.`Custom`

- `data-center` `ID`
- `worker` `ID`

```rust
let center_id = 16;
let worker_id = 16;

let gen = SnowflakeGenerator::new(center_id, worker_id);
assert!(gen.is_ok());
let rvt = gen.unwrap().next_id();
assert!(rvt.is_ok());
```

-- -

## 3.`Test`

### 3.1.`cargo test`

```shell
$ cargo test --features "dynamic" -- --show-output
$ cargo test --features "dynamic"
```

-- -

## 4.`Docs`

### 4.1.`features`

```shell
$ cargo doc --open --features dynamic
```

