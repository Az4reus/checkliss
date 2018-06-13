# v1.0, things left to do: 

## Feature: Sublist parsing
- Fix mixed-indentation file parsing test, should fail with parsing error
  because there's a not-indented, top-level item that cannot be an item because
  it lacks a leading hyphen but also cannot be parsed to a sublist because of
  its lack of indentation to two spaces/one tab
- Actually specify the grammar rules required to form an item.
- Change parsing to be not about lines, but about a stream of tokens (text,
  linebreak, etc, actually make it a parser) 

## Feature: Enumerating of List items. 

- I want it so every top level carries a number, with sublists carrying a
  smaller version of that number. 
- This progression goes `1, 1.1, 1.1.1`, etc. 
- This number is kept track of while rendering, not in the item itself. 

## Feature: Dating of lists, with file name, in a tiny footer

- Function that formats current date appropriately
- Implementation of a `-c/--clean` flag that surpresses this footer

## Bugfixes

- Fix handling of the `-o` flag to determine outputs. Currently fails with "Not
  expected" 
- Change default font away from Roboto Slab to just default LaTeX font, as
  Roboto Slab is probably not actually installed in places leading to errors. 
- Clean up after ourselves. We currently aren't.

# Potential Future Features

- CLI Flag to set font used
