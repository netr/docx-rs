# docx-rust Library Improvements for Table Support

**Repository**: https://github.com/netr/docx-rs
**Branch**: `fix/alternate-content`
**Date**: 2025-10-14

## Overview

The docx-rust library is missing critical table cell properties needed for proper table parsing:
- `gridSpan` (for colspan/horizontal cell merging)
- `vMerge` (for rowspan/vertical cell merging)

These properties exist in DOCX XML but are not exposed by the library.

## Required Changes

### 1. Create New Type: `GridSpan`

**File**: `src/formatting/grid_span.rs` (NEW FILE)

```rust
use hard_xml::{XmlRead, XmlWrite};

use crate::{__setter, __xml_test_suites};

/// Grid Span (Colspan)
/// Specifies the number of columns this cell spans horizontally
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:gridSpan")]
pub struct GridSpan {
    #[xml(attr = "w:val")]
    pub val: Option<usize>,
}

impl GridSpan {
    __setter!(val: Option<usize>);
}

impl From<usize> for GridSpan {
    fn from(val: usize) -> Self {
        GridSpan { val: Some(val) }
    }
}

__xml_test_suites!(
    GridSpan,
    GridSpan::default(),
    "<w:gridSpan/>",
    GridSpan::from(2),
    r#"<w:gridSpan w:val="2"/>"#,
    GridSpan::from(4),
    r#"<w:gridSpan w:val="4"/>"#,
);
```

### 2. Create New Type: `VMerge`

**File**: `src/formatting/v_merge.rs` (NEW FILE)

```rust
use hard_xml::{XmlRead, XmlWrite};

use crate::{__setter, __string_enum, __xml_test_suites};

/// Vertical Merge (Rowspan)
/// Specifies whether this cell is merged vertically with cells above/below
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:vMerge")]
pub struct VMerge {
    #[xml(attr = "w:val")]
    pub val: Option<VMergeType>,
}

impl VMerge {
    __setter!(val: Option<VMergeType>);
}

impl From<VMergeType> for VMerge {
    fn from(val: VMergeType) -> Self {
        VMerge { val: Some(val) }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum VMergeType {
    Restart, // Start of a merged cell region
    Continue, // Continuation of merged cell region (cell is hidden)
}

impl Default for VMergeType {
    fn default() -> Self {
        VMergeType::Restart
    }
}

__string_enum! {
    VMergeType {
        Restart = "restart",
        Continue = "continue",
    }
}

__xml_test_suites!(
    VMerge,
    VMerge::default(),
    "<w:vMerge/>",
    VMerge::from(VMergeType::Restart),
    r#"<w:vMerge w:val="restart"/>"#,
    VMerge::from(VMergeType::Continue),
    r#"<w:vMerge w:val="continue"/>"#,
);
```

### 3. Update `TableCellProperty`

**File**: `src/formatting/table_cell_property.rs`

**Current code**:
```rust
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tcPr")]
pub struct TableCellProperty {
    #[xml(child = "w:tcW")]
    pub wide: Option<super::TableCellWidth>,
    #[xml(default, child = "w:vAlign")]
    pub v_align: super::VAlign,
}

impl TableCellProperty {
    __setter!(v_align: super::VAlign);
    __setter!(wide: Option<super::TableCellWidth>);
}
```

**Change to**:
```rust
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tcPr")]
pub struct TableCellProperty {
    #[xml(child = "w:tcW")]
    pub wide: Option<super::TableCellWidth>,
    #[xml(child = "w:gridSpan")]
    pub grid_span: Option<super::GridSpan>,
    #[xml(child = "w:vMerge")]
    pub v_merge: Option<super::VMerge>,
    #[xml(default, child = "w:vAlign")]
    pub v_align: super::VAlign,
}

impl TableCellProperty {
    __setter!(v_align: super::VAlign);
    __setter!(wide: Option<super::TableCellWidth>);
    __setter!(grid_span: Option<super::GridSpan>);
    __setter!(v_merge: Option<super::VMerge>);
}
```

### 4. Update `mod.rs` Exports

**File**: `src/formatting/mod.rs`

Add these lines to the module declarations section:
```rust
mod grid_span;
mod v_merge;
```

Add to the public exports:
```rust
pub use self::{
    // ... existing exports ...
    grid_span::*, v_merge::*,
    // ... rest of exports ...
};
```

## Verification

### XML Examples Found in Real DOCX Files

From `tables_complex.docx`:
```xml
<!-- Colspan example -->
<w:tc>
  <w:tcPr>
    <w:gridSpan w:val="4"/>
    <w:vAlign w:val="top"/>
  </w:tcPr>
  ...
</w:tc>

<!-- Rowspan start -->
<w:tc>
  <w:tcPr>
    <w:vMerge w:val="restart"/>
    <w:vAlign w:val="top"/>
  </w:tcPr>
  ...
</w:tc>

<!-- Rowspan continuation (hidden cell) -->
<w:tc>
  <w:tcPr>
    <w:vMerge w:val="continue"/>
    <w:vAlign w:val="top"/>
  </w:tcPr>
  ...
</w:tc>

<!-- Combined: colspan + rowspan -->
<w:tc>
  <w:tcPr>
    <w:gridSpan w:val="2"/>
    <w:vMerge w:val="restart"/>
    <w:vAlign w:val="top"/>
  </w:tcPr>
  ...
</w:tc>
```

## Table Alignment (Already Supported!)

**Good news**: Table alignment is already supported via `TableProperty`.

**File**: Already exists in `src/formatting/table_property.rs`

**Field**: `justification: Option<TableJustification>`

**Access**: `table.property.justification.value` returns:
- `Some(Left)`
- `Some(Center)`
- `Some(Right)`
- `Some(Both)` (justified)
- `None`

## Implementation Notes

1. **Field Order**: In `TableCellProperty`, fields should match XML element order:
   - `wide` (tcW)
   - `grid_span` (gridSpan)
   - `v_merge` (vMerge)
   - `v_align` (vAlign)

2. **Optional Fields**: All new fields are `Option<T>` because they're not always present

3. **Testing**: Each new type includes XML round-trip tests via `__xml_test_suites!` macro

4. **Pattern**: Follow existing types like `TableCellWidth` and `VAlign` as templates

5. **Compatibility**: Changes are additive only - no breaking changes to existing API

## Usage After Implementation

### In Lacuna (consumer code)

```rust
// Extract colspan
let colspan = cell.property.grid_span
    .and_then(|gs| gs.val)
    .map(|v| v as u32);

// Extract rowspan
let rowspan = match cell.property.v_merge {
    Some(ref vm) => match vm.val {
        Some(VMergeType::Restart) => Some(1), // Start of merge
        Some(VMergeType::Continue) => None,   // Hidden cell
        None => None,
    },
    None => None,
};

// Extract table alignment
let alignment = table.property.justification
    .and_then(|j| j.value)
    .map(|v| match v {
        TableJustificationType::Left => Alignment::Left,
        TableJustificationType::Center => Alignment::Center,
        TableJustificationType::Right => Alignment::Right,
        TableJustificationType::Both => Alignment::Justified,
    });
```

## Testing the Changes

After implementing, test with `tables_complex.docx` which contains:
- Row 0: colspan=4 (header spanning all columns)
- Row 1, Cell 2: colspan=2 (header spanning 2 columns)
- Row 2, Cell 0: vMerge="restart" (start of 2-row merge)
- Row 3, Cell 0: vMerge="continue" (continuation, hidden)
- Row 2, Cell 2: colspan=2 (data cell spanning 2 columns)
- Row 4: colspan=4 (footer spanning all columns)

Expected cell counts per row:
- Row 0: 1 cell (gridSpan=4)
- Row 1: 3 cells (regular, regular, gridSpan=2)
- Row 2: 3 cells (vMerge=restart, regular, gridSpan=2)
- Row 3: 4 cells (vMerge=continue, 3x regular)
- Row 4: 1 cell (gridSpan=4)

## Files Summary

**New files to create:**
1. `src/formatting/grid_span.rs`
2. `src/formatting/v_merge.rs`

**Files to modify:**
1. `src/formatting/table_cell_property.rs` - Add 2 new fields
2. `src/formatting/mod.rs` - Add 2 module declarations and exports

**Total changes:** 2 new files, 2 modified files

## References

- DOCX Spec: [ECMA-376](http://www.ecma-international.org/publications-and-standards/standards/ecma-376/)
- Element: `w:gridSpan` (ISO/IEC 29500-1:2016, §17.4.17)
- Element: `w:vMerge` (ISO/IEC 29500-1:2016, §17.4.85)
- Element: `w:jc` / `w:tblPr/w:jc` (ISO/IEC 29500-1:2016, §17.4.27)
