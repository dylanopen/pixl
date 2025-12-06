# Migration guides

These guides should help you get up to date when you're upgrading from one
version to another which contains breaking changes.

Breaking changes are included in ***major*** versions (see
[Semantic Versioning](https://semver.org/)). For example, `0.1.2 -> 0.2.0` is a
breaking change, while `0.1.2 -> 0.1.3` is not.  
When Pixl has a stable `1.0.0` release, breaking changes will only be
introduced in `X.0.0` releases (e.g. `1.2.3 -> 2.0.0`).

## Migrating from 0.1.x to 0.2.x

- The `PositionComponent` type now has its `x` and `y` fields as `f64` instead
  of `usize`. Replace any calls in/to nodes implementing this accordingly.
- The `SizeComponent` type now has its `width` and `height` fields as `f64`
  instead of `usize`. Replace any calls in/to nodes implementing this
  accordingly.

### Example migration

Before:

```rust
RectangleNode::new(10, 20, 100, 50);
CircleNode::new(30, 40, 25);
```

After:

```rust
RectangleNode::new(10.0, 20.0, 100.0, 50.0);
CircleNode::new(30.0, 40.0, 25);
```

