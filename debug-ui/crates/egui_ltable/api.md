# Table widget

- [x] Header row and multiple rows
- [x] Multiple columns
- [x] Vertical and horizontal scrolling
- [x] Fixed rows and columns that are always visible. Even when scrolled
    - [x] Fixed row
    - [x] Fixed columns
- [x] Clickable row/column/cell
- [x] Resizeable columns
- [x] Automatically sized columns
- [x] Row highlighting (every second row and/or user chosen)
- [x] Differing row hights
- [x] Layout per cell or column
- [ ] Row/column/cell selection
- [ ] Reorder column


### API:
```rs
use egui_ltable::*;

Table::default()
    .reorder_columns(false)
    .highlight_even_rows(true)
    .column(Column::initial(150.0).min(100.0).max(500.0))
    .column(Column::fill(1.0).resizable(true))
    .column(Column::auto())
    .resizeable_headers_only(true)
    .show(|table|{
        // Headers
        table.row(Row::header()
            .height(40.0)
            .highlight(true)
            .fixed(true),|row|{
                row.next_cells(|ui| {
                    ui.label("Column 1");
                });
                row.next_cells(|ui| {
                    ui.label("Column 2");
                });
            });
        Row::header()
            .height(40.0)
            .highlight(true)
            .fixed(true)
            .show(table, |row|{
                row.next_cells(|ui| {
                    ui.label("Column 1");
                });
                row.next_cells(|ui| {
                    ui.label("Column 2");
                });
            });
        // Data
        Row::single()
            .height(40.0)
            .fixed(true)
            .show(table,|row|{
                row.cell(|ui|{
                    ui.label("cell 1");
                });
                row.cell(|ui|{
                    ui.label("cell 2");
                });
            });
    });
```

### Sizing:
A table tries to be as small as it can normaly. If it is bigger than the current frame then it will overflow.
It can be expanded to take up the remaining horizontal or vertical space.
If the table is bigger then, it will show scroll bars.

Column sizing:
- `auto` the column is big enough to fit the content of the column.
- `exact` the column is exactly this big
- `initial` the inial size of the column
- `min_width`, `max_width` set the minimum and maximum width of the column.
- `fill_space` will expand the column size to make the table fill the entire horizonal width.  
    This only works if the table itself is filling the available width.  
    If the table isnt, then the column returns to default sizing.  
- `resizeable` whether or not the column can be resized by the user.1

Find column size
If table fill horizontal:
Static width is the sum of all column widths that are not `fill_space`.
Fill width is the available width minus the static width.
Fill width is devided among the `fill_space` columns respecting the minimum width.
Table width is the sum of all column widths.

If table is not fill horizontal:
Table width is the sum of all column widths.



