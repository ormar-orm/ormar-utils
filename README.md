# ormar-utils

Rust-accelerated utility functions for the [ormar](https://github.com/ormar-orm/ormar) async ORM.

This package provides optional Rust implementations of performance-critical operations used internally by ormar. When installed, ormar automatically uses these faster implementations.

## Installation

```bash
pip install ormar-utils
```

Or install ormar with the `rust` extra:

```bash
pip install ormar[rust]
```

## Requirements

- Python >= 3.10
- A Rust toolchain (for building from source)

Wheels are built against the CPython stable ABI (`abi3`, Python 3.10+), so a
single wheel per platform works on current and future Python releases without a
per-version rebuild.

## API Reference

All functions are exposed from the `ormar_rust_utils` module:

### Parsers
- `encode_bytes(value, represent_as_string=False)` - Encode bytes to string (UTF-8 or base64)
- `decode_bytes(value, represent_as_string=False)` - Decode string to bytes (UTF-8 or base64)
- `encode_json(value)` - Encode a value to JSON string

### Hashing
- `hash_item(item)` - Convert a dict/list into a hashable tuple for use as dict key

### List/Dict Utilities
- `translate_list_to_dict(list_to_trans, default=None)` - Split `__`-separated strings into nested dict
- `group_related_list(list_)` - Group related strings into nested dictionary

### Collections
- `UniqueList(initial=None)` - A list that prevents duplicates using hash-based O(1) lookups

### Alias Utilities
- `build_reverse_alias_map(field_alias_map)` - Build a cached alias -> field_name lookup (with identity entries) from a field_name -> alias mapping

### Merge Infrastructure
- `group_by_pk(pks)` - Group items by PK hash, preserving insertion order
- `plan_merge_items_lists(current_pks, other_pks)` - Create a merge plan for two lists by PK

## License

MIT
