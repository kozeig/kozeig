# Kozeig Language Changelog

## Version 0.1.0 - A Star is Reborn

- Initial release
- Supports:
  - types - both implicit and explicit (`number`, `fp`, `bool`, `text`, `array`) with more to come
  - operators (`+`, `-`, `*`, `/`, `%`, `^`, `++`, `--`, `==`, `!=`, `<`, `>`, `<=`, `>=`, `&&`, `||`, `!`, `?`)
  - functions using the `func pub|prot <name> { args } []` syntax
    - functions can be recursive
  - local dependencies/module import with a `use-from` statement
