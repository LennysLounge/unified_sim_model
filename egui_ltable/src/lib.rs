use std::{collections::VecDeque, ops::RangeInclusive};

use egui::{
    pos2, vec2, Color32, Id, Layout, NumExt, Painter, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2,
};

#[derive(Default, Debug, Clone)]
struct TableState {
    columns: Vec<ColumnState>,
}

impl TableState {
    fn load(ui: &Ui, state_id: Id) -> Self {
        let rect = Rect::from_min_size(ui.cursor().min, Vec2::ZERO);
        ui.ctx().check_for_id_clash(state_id, rect, "Table");

        if let Some(state) = ui.data_mut(|d| d.get_persisted::<Self>(state_id)) {
            state
        } else {
            TableState::default()
        }
    }

    fn store(self, ui: &Ui, state_id: Id) {
        ui.data_mut(|d| d.insert_persisted(state_id, self));
    }
}

#[derive(Clone, Copy, Debug)]
struct ColumnState {
    width: f32,
    pos: i32,
}

//  -------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct Row {
    /// The height of the row.
    height: f32,
    /// If the row is fixed to the visible area.
    fixed: bool,
    /// The interaction sense of this row.
    sense: Sense,
    /// If the row should be highlighted if hovered.
    hover_highlight: bool,
    /// If the row should be highlighted.
    highlight: bool,
}

impl Row {
    pub fn new() -> Self {
        Self {
            height: 40.0,
            fixed: false,
            sense: Sense::hover(),
            hover_highlight: false,
            highlight: false,
        }
    }

    /// Set the height of the row.
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set the row to be fixed. This will make sure that this row is always
    /// visible in the viewport when the table is scrolled.
    ///
    /// If vertical scrolling is disabled, this setting will have no effect
    /// since all rows are always visible.
    pub fn fixed(mut self, fixed: bool) -> Self {
        self.fixed = fixed;
        self
    }

    /// Set the sense level the row should respond to.
    ///
    /// A sense other than `Sense::hover()` will stop the
    /// table from beeing dragged while in a scroll area.
    ///
    /// Default is `Sense::hover()`.
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = sense;
        self
    }

    /// Set if the row should be highlighted when hovered.
    pub fn hover_highlight(mut self, hover_highlight: bool) -> Self {
        self.hover_highlight = hover_highlight;
        self
    }

    /// Set the row to be highlighted.
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = highlight;
        self
    }
}

/// Configure a table column.
#[derive(Clone, Default, Debug)]
pub struct Column {
    /// The width of the column the first time it is shown.
    initial_width: Option<f32>,
    /// Minium width of the column.
    min_width: f32,
    /// Maximum widht of the column
    max_width: f32,
    /// How much of the available space it takes up.
    fill_share: Option<f32>,
    /// If the column is resizeable.
    resizeable: bool,
    /// If the column is fixed to the viewport.
    fixed: bool,
    /// The layout to use for this column.
    layout: Layout,
}

impl Column {
    /// Create a column that is automatically sized to fit its content.
    ///
    /// The sizing only happens the first time the column is displayed.
    ///
    /// Equivalent to:
    /// ```rs
    /// Column::new()
    /// ```
    pub fn auto() -> Self {
        Self {
            initial_width: None,
            min_width: 20.0,
            max_width: f32::INFINITY,
            fill_share: None,
            resizeable: false,
            fixed: false,
            layout: Layout::left_to_right(egui::Align::Min).with_main_wrap(false),
        }
    }

    /// Create a column with an exact width.
    ///
    /// Equivalent to:
    /// ```rs
    /// Column::auto()
    ///     .initial_width(width)
    ///     .min_width(width)
    ///     .max_width(width)
    ///     .resizeable(false)
    /// ```
    pub fn exact(width: f32) -> Self {
        Column::auto()
            .initial_width(width)
            .max_width(width)
            .min_width(width)
            .resizeable(false)
    }

    /// Create a column with an initial width that is resizeable.
    pub fn initial(width: f32) -> Self {
        Column::auto().initial_width(width)
    }

    /// Create a column that will fill the available space.
    ///
    /// The fill share indicates how much space this column will
    /// take up compared to other columns that fill space.
    pub fn fill(fill_share: f32) -> Self {
        Column::auto().fill_share(fill_share)
    }

    /// Set the initial width of the column.
    ///
    /// Default is `None` in which case the width of the header
    /// is used.
    pub fn initial_width(mut self, width: f32) -> Self {
        self.initial_width = Some(width);
        self
    }

    /// Set the minimum width of the column.
    ///
    /// Default is 20.0.
    pub fn min_width(mut self, min_width: f32) -> Self {
        self.min_width = min_width;
        self
    }

    /// Set the maximum width of the column.
    ///
    /// Default is `f32::INFINITY`
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }
    /// Set the column to expand and fill available space.
    ///
    /// `fill_share` indicates how much of the availabe space is take up by this
    /// column. If this column gets resized, the internal fill share is also
    /// updated to reflect the change.
    pub fn fill_share(mut self, fill_share: f32) -> Self {
        self.fill_share = Some(fill_share);
        self
    }

    /// Set the column to be resizeable.
    pub fn resizeable(mut self, resizeable: bool) -> Self {
        self.resizeable = resizeable;
        self
    }

    /// Set the column to be fixed. A fixed column is always visible
    /// in the viewport. Has no effect if horizonzal scrolling is disabled since
    /// all columns are always visible.
    pub fn fixed(mut self, fixed: bool) -> Self {
        self.fixed = fixed;
        self
    }

    /// Set the layout to use for this column.
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    fn is_auto_sized(&self) -> bool {
        self.fill_share.is_none() && self.initial_width.is_none()
    }
}

//  -------------------------------------------------------------------------------------

/// The complete layout of the table.
struct TableLayout {
    /// List of the columns in the table.
    columns: Vec<ColumnLayout>,
    /// The full size of the table alone.
    rect: Rect,
    /// The visible area of the table. The clip may be bigger than the
    /// table itself. Onyl areas of the table that intersect with this
    /// rect are visible.
    clip: Rect,
    /// The area of the table that is not occupied by fixed rows/columns.
    /// The free viewport is the middle area of the table where the rows
    /// and columns can freely scroll.
    /// Fixed rows and columns are outside of this rect.
    free_viewport: Rect,
}

/// The layout of a column.
#[derive(Default, Debug, Clone)]
struct ColumnLayout {
    /// The column definition.
    definition: Column,
    /// The horizontal position of the column.
    x_pos: f32,
    /// The calculated width of the column.
    width: f32,
    /// The position in the table.
    pos_index: i32,
    /// The width of the content in the column
    content_width: f32,
    /// If and where the column was fixed to.
    fixed: ColumnFixed,
    /// If this is the first time the column is layed out.
    first_time: bool,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
enum ColumnFixed {
    #[default]
    None,
    Left,
    Right,
}

pub struct Table {
    /// The list of defined columns.
    columns: Vec<Column>,
    /// If horizontal scroll is enabled.
    h_scroll: bool,
    /// If vertical scroll is enabled.
    v_scroll: bool,
    /// If every odd row should be highlighted.
    striped: bool,
    /// If lines seperating the columns are enabled.
    column_lines: bool,
    /// If resizing of rows is possible for the entire height of the
    /// table or only for the header row.
    resize_full_height: bool,
}

impl Table {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            h_scroll: false,
            v_scroll: false,
            striped: false,
            column_lines: false,
            resize_full_height: true,
        }
    }

    /// Add a column to the table.
    pub fn column(mut self, column: Column) -> Self {
        self.columns.push(column);
        self
    }

    /// Set the scrollig behavior of the table.
    pub fn scroll(mut self, h_scroll: bool, v_scroll: bool) -> Self {
        self.h_scroll = h_scroll;
        self.v_scroll = v_scroll;
        self
    }

    /// Whether or not odd rows are highlighted.
    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    /// Whether to draw lines seperating the columns or not.
    pub fn column_lines(mut self, lines: bool) -> Self {
        self.column_lines = lines;
        self
    }

    /// Whether or not the resizing of rows can be done along the entire
    /// height of the table or only on the header rows.
    pub fn resize_full_height(mut self, full_height: bool) -> Self {
        self.resize_full_height = full_height;
        self
    }

    pub fn show(mut self, ui: &mut Ui, add_body_content: impl FnOnce(&mut Body)) {
        let mut child_ui = ui.child_ui(ui.available_rect_before_wrap(), *ui.layout());
        child_ui.style_mut().spacing.scroll_bar_inner_margin = 0.0;

        let top_left = ui.cursor().min;
        match (self.h_scroll, self.v_scroll) {
            (true, true) => egui::ScrollArea::both(),
            (true, false) => egui::ScrollArea::horizontal(),
            (false, true) => egui::ScrollArea::vertical(),
            (false, false) => egui::ScrollArea::neither(),
        }
        .auto_shrink([true, true])
        .show(&mut child_ui, |ui| {
            let width = ui.available_width();
            let height = ui.available_height();
            let clip = Rect::from_min_size(
                top_left,
                match (self.h_scroll, self.v_scroll) {
                    (true, true) => vec2(width, height),
                    (true, false) => vec2(width, f32::INFINITY),
                    (false, true) => vec2(f32::INFINITY, height),
                    (false, false) => vec2(f32::INFINITY, f32::INFINITY),
                },
            );

            self.show_body(clip, ui, add_body_content);
        });

        ui.allocate_rect(child_ui.min_rect(), Sense::hover());
    }

    fn show_body(&mut self, clip: Rect, ui: &mut Ui, add_body_content: impl FnOnce(&mut Body)) {
        let state_id = ui.id().with("_table_state");
        let table_state = TableState::load(ui, state_id);

        let table_layout = self.layout_columns(
            &table_state,
            ui.cursor().min,
            // subtract one from the width to avoid scrollbar problems from floating point rounding errors.
            ui.available_width() - 1.0,
            clip,
        );

        let mut table_body = Body {
            table_layout,
            cursor: ui.cursor().min,
            ui,
            row_count: 0,
            striped: self.striped,
        };
        add_body_content(&mut table_body);
        let Body {
            mut table_layout,
            cursor,
            ..
        } = table_body;

        // Allocate space for the table.
        table_layout.rect.set_bottom(cursor.y);
        ui.allocate_rect(table_layout.rect, Sense::hover());

        // Set the maximum height for the freeport.
        table_layout.free_viewport =
            constrain_top_bottom(table_layout.free_viewport, table_layout.rect);

        // The rectangle of the table that is visible.
        self.resize_columns(ui, &mut table_layout);

        // Save the column state
        self.save_column_widths(ui, state_id, &table_layout.columns);
    }

    fn resize_columns(&mut self, ui: &mut Ui, table_layout: &mut TableLayout) {
        /*
        Few notes about the implementation of this since it is a bit tricky to get correct.

        1) Each sense rectable has its own id to keep track of its state internally.
        Therefore we must allocate something for each column even if we dont plan on sensing anything.
        If we dont, when a column disappears the state of that id remains and now a different column is affected.

        2) We always have to draw the seperators for the fixed columns first. If we dont, it messes with the ids again.
        */
        let sense_width = 5.0;

        let (line_range, interact_range) = {
            let table_visible_area = constrain_to(table_layout.rect, table_layout.clip);
            let mut header_rect = table_visible_area;
            header_rect.set_bottom(table_layout.free_viewport.top());

            let full_height = table_visible_area.y_range();
            let header_only = header_rect.y_range();

            match (self.column_lines, self.resize_full_height) {
                (true, true) => (full_height.clone(), full_height),
                (true, false) => (full_height.clone(), header_only),
                (false, true) => (full_height.clone(), full_height),
                (false, false) => (header_only.clone(), header_only),
            }
        };

        let visible_range = table_layout.free_viewport.x_range();
        let sense_range = {
            let left_offset = table_layout
                .columns
                .iter()
                .any(|col| col.fixed == ColumnFixed::Left)
                .then_some(visible_range.start() + sense_width * 2.0);
            let right_offset = table_layout
                .columns
                .iter()
                .any(|col| col.fixed == ColumnFixed::Right)
                .then_some(visible_range.end() - sense_width * 2.0);
            RangeInclusive::new(
                left_offset.unwrap_or(f32::NEG_INFINITY),
                right_offset.unwrap_or(f32::INFINITY),
            )
        };

        let mut fixed_columns_first: Vec<_> = table_layout.columns.iter_mut().collect();
        fixed_columns_first.sort_by(|c1, c2| c2.definition.fixed.cmp(&c1.definition.fixed));
        for column in fixed_columns_first.iter_mut() {
            // position of the resize bar and the direction of the drag.
            let (pos, dir, is_fixed) = match column.fixed {
                ColumnFixed::None => (column.x_pos + column.width, 1.0, false),
                ColumnFixed::Left => (column.x_pos + column.width, 1.0, true),
                ColumnFixed::Right => (column.x_pos, -1.0, true),
            };

            let is_visible = visible_range.contains(&pos) || is_fixed;
            if !is_visible {
                continue;
            }

            // Draw seperator line
            if column.definition.resizeable {
                ui.painter().vline(
                    pos,
                    interact_range.clone(),
                    Stroke::new(3.0, ui.visuals().noninteractive().bg_stroke.color),
                );
            }
            if self.column_lines {
                ui.painter().vline(
                    pos,
                    line_range.clone(),
                    ui.visuals().noninteractive().bg_stroke,
                );
            }

            let has_sense = sense_range.contains(&pos) || is_fixed;
            if !column.definition.resizeable || !has_sense {
                ui.allocate_rect(Rect::NOTHING, Sense::hover());
                continue;
            }

            let sense_rect = Rect::from_min_max(
                pos2(pos - sense_width, *interact_range.start()),
                pos2(pos + sense_width, *interact_range.end()),
            );

            let sense = ui
                .allocate_rect(sense_rect, Sense::click_and_drag())
                .on_hover_cursor(egui::CursorIcon::ResizeColumn);

            // highlight the column that is going to be resized if dragged.
            if sense.hovered() || sense.dragged() {
                let mut column_highlight = Rect::from_min_max(
                    pos2(column.x_pos, *interact_range.start()),
                    pos2(column.x_pos + column.width, *interact_range.end()),
                );
                if !is_fixed {
                    column_highlight =
                        constrain_left_right(column_highlight, table_layout.free_viewport);
                }
                ui.painter().rect_filled(
                    column_highlight,
                    0.0,
                    Color32::from_rgba_unmultiplied(255, 255, 255, 2),
                );
                // Draw seperator line
                ui.painter().vline(
                    pos,
                    interact_range.clone(),
                    Stroke::new(3.0, ui.visuals().widgets.hovered.bg_stroke.color),
                );
            }
            if sense.dragged() {
                let (max_drag, min_drag) = match is_fixed {
                    false => (sense_range.end() - pos, sense_range.start() - pos),
                    true => (f32::INFINITY, f32::NEG_INFINITY),
                };
                let drag_delta = sense.drag_delta().x.max(min_drag).min(max_drag);
                column.width += drag_delta * dir;

                // Draw seperator line
                ui.painter().vline(
                    pos,
                    interact_range.clone(),
                    Stroke::new(3.0, ui.visuals().widgets.active.bg_stroke.color),
                );
            }
        }
    }

    fn save_column_widths(&mut self, ui: &Ui, state_id: Id, column_layout: &Vec<ColumnLayout>) {
        let mut new_table_state = TableState::default();
        for (i, column) in column_layout.iter().enumerate() {
            let width = if column.first_time && column.definition.is_auto_sized() {
                println!("Save column {} with content width: {}", i, column.content_width);
                column.content_width
            } else {
                column.width
            };
            new_table_state.columns.push(ColumnState {
                width: width
                    .at_least(column.definition.min_width)
                    .at_most(column.definition.max_width),
                pos: column.pos_index,
            });
        }
        TableState::store(new_table_state, ui, state_id);
    }

    fn layout_columns(
        &mut self,
        table_state: &TableState,
        table_origin: Pos2,
        available_width: f32,
        max_clip_rect: Rect,
    ) -> TableLayout {
        let mut layout = Vec::new();
        for (i, col) in self.columns.iter().enumerate() {
            let (pos_index, width, first_time) = table_state
                .columns
                .get(i)
                .map(|state| (state.pos, state.width, false))
                .unwrap_or((
                    i as i32,
                    col.fill_share.or(col.initial_width).unwrap_or(0.0),
                    true,
                ));
            layout.push(ColumnLayout {
                definition: col.clone(),
                width,
                pos_index,
                first_time,
                ..Default::default()
            });
        }

        layout = self.calculate_column_widths(available_width, layout);

        let width: f32 = layout.iter().map(|col| col.width).sum();
        let table_rect = Rect::from_min_size(table_origin, vec2(width, 0.0));
        let clip = constrain_left_right(max_clip_rect, table_rect);
        let mut free_viewport = clip.clone();
        if free_viewport.width() > width {
            free_viewport.set_right(free_viewport.left() + width);
        }

        let mut layout_ref: Vec<_> = layout.iter_mut().collect();
        // Calculate position and fix columns
        let mut pos = table_origin.x;
        layout_ref.sort_by(|c1, c2| c1.pos_index.cmp(&c2.pos_index));
        for column in layout_ref.iter_mut() {
            column.x_pos = pos;
            pos += column.width;

            if column.definition.fixed {
                if column.x_pos <= free_viewport.left() {
                    column.x_pos = free_viewport.left();
                    column.fixed = ColumnFixed::Left;
                    *free_viewport.left_mut() += column.width;
                }
            };
        }

        // fix columns on the right.
        layout_ref.reverse();
        for column in layout_ref {
            if column.definition.fixed {
                if column.x_pos + column.width >= free_viewport.right() {
                    column.x_pos = free_viewport.right() - column.width;
                    column.fixed = ColumnFixed::Right;
                    *free_viewport.right_mut() -= column.width;
                }
            };
        }

        TableLayout {
            columns: layout,
            clip,
            free_viewport: free_viewport,
            rect: table_rect,
        }
    }

    fn calculate_column_widths(
        &mut self,
        available_width: f32,
        initial_layout: Vec<ColumnLayout>,
    ) -> Vec<ColumnLayout> {
        // Goals of this algorithm:
        // - Every column must respect its min and max widths.
        // - Two columns that have the same fill percent and are not limited by their min and max width
        //      must have the same width
        //
        // This algorithm is a breadth first search of all the possibilities of
        // how this table could be layed out.
        // We layout the columns and then we check if any column violates its min/max widths
        // That column is then resized to fit its limits and fixed in place. The remaining columns
        // are then layouted again with the same procedure.
        // If multiple columns violate their limits, then we need to check for each column, if
        // fixing that column will create a valid layout. The order in which invalid columns are
        // fixed matters. So we explore every variation until we find one that creates a valid layout.
        // Doing a breadth first search guarantees that we find a layout with the smallest amount of changes
        // necessary to create a valid layout.
        #[derive(Default, Clone, Debug)]
        struct LayoutRun {
            layout: Vec<ColumnLayout>,
            active_columns: Vec<usize>,
            dynamic_width: f32,
            total_fill: f32,
        }

        let mut first_run = LayoutRun {
            layout: initial_layout.clone(),
            dynamic_width: available_width,
            ..Default::default()
        };
        for (i, column) in initial_layout.iter().enumerate() {
            if column.definition.fill_share.is_some() {
                first_run.active_columns.push(i);
                first_run.total_fill += column.width;
            } else {
                first_run.dynamic_width -= column.width;
            }
        }

        // There are no columns with fill.
        if first_run.active_columns.is_empty() {
            return initial_layout;
        }

        let mut openlist = VecDeque::new();
        openlist.push_back(first_run);

        while let Some(mut run) = openlist.pop_front().take() {
            // Calculate the widths for all open columns.
            let mut all_columns_fit = true;
            for (i, column_index) in run.active_columns.iter().copied().enumerate() {
                let column = &mut run.layout[column_index];
                let fill_share = column.width;
                column.width = run.dynamic_width * fill_share / run.total_fill;

                if column.width < column.definition.min_width
                    || column.width > column.definition.max_width
                {
                    let width = column
                        .width
                        .at_least(column.definition.min_width)
                        .at_most(column.definition.max_width);
                    column.width = width;

                    let mut new_run = run.clone();
                    new_run.dynamic_width -= width;
                    new_run.total_fill -= fill_share;
                    new_run.active_columns.remove(i);
                    openlist.push_back(new_run);
                    all_columns_fit = false;
                }
            }

            // Move widths to the columns and return.
            if all_columns_fit {
                return run.layout.clone();
            }
        }
        initial_layout
    }
}

pub struct Body<'a> {
    ui: &'a mut Ui,
    table_layout: TableLayout,
    cursor: Pos2,
    row_count: i32,
    striped: bool,
}

impl<'a> Body<'a> {
    pub fn row(&mut self, row: Row, add_row_content: impl FnOnce(&mut RowUi)) -> Response {
        let row_rect = self.get_row_rect(row);

        let mut row_viewport = constrain_to(row_rect, self.table_layout.clip);
        if !row.fixed {
            row_viewport = constrain_top_bottom(row_viewport, self.table_layout.free_viewport);
        };

        let response = self.ui.allocate_rect(row_viewport, row.sense);

        let mut row_ui = RowUi {
            body: self,
            config: row,
            cell_count: 0,
            rect: row_rect,
            cell_was_hovered: false,
        };
        add_row_content(&mut row_ui);
        let RowUi {
            cell_was_hovered, ..
        } = row_ui;

        if row.fixed {
            self.adjust_viewport(row.height);
        }
        self.cursor.y += row.height;
        self.row_count += 1;

        // Draw highlight
        if row.highlight {
            self.ui.painter().rect_filled(
                row_viewport,
                00.0,
                self.ui.visuals().faint_bg_color.linear_multiply(4.0),
            );
        }
        if row.hover_highlight && (was_hoverd_strict(&response) || cell_was_hovered) {
            self.ui.painter().rect_filled(
                row_viewport,
                0.0,
                self.ui.visuals().faint_bg_color.linear_multiply(4.0),
            );
        }

        Response {
            hovered: was_hoverd_strict(&response),
            ..response
        }
    }

    fn get_row_rect(&self, row: Row) -> Rect {
        let mut row_viewport = Rect::from_min_size(self.cursor, vec2(f32::INFINITY, row.height));
        if row.fixed {
            if row_viewport.top() <= self.table_layout.free_viewport.top() {
                row_viewport = row_viewport.translate(vec2(
                    0.0,
                    self.table_layout.free_viewport.top() - row_viewport.top(),
                ));
            }
            if row_viewport.bottom() > self.table_layout.free_viewport.bottom() {
                row_viewport = row_viewport.translate(vec2(
                    0.0,
                    self.table_layout.free_viewport.bottom() - row_viewport.bottom(),
                ));
            }
        }
        row_viewport
    }

    fn adjust_viewport(&mut self, height: f32) {
        if self.cursor.y <= self.table_layout.free_viewport.top() {
            *self.table_layout.free_viewport.top_mut() += height;
        }
        if self.cursor.y + height > self.table_layout.free_viewport.bottom() {
            *self.table_layout.free_viewport.bottom_mut() -= height;
        }
    }
}

pub struct RowUi<'a, 'b> {
    body: &'a mut Body<'b>,
    config: Row,
    cell_count: usize,
    rect: Rect,
    cell_was_hovered: bool,
}

impl<'a, 'b> RowUi<'a, 'b> {
    /// Add the next cell in this row.
    pub fn cell<R>(&mut self, add_content: impl FnOnce(&mut Ui) -> R) -> Option<Response> {
        self.cell_sense(Sense::hover(), add_content)
    }

    /// Add the next cell to this row with sense.
    pub fn cell_sense<R>(
        &mut self,
        sense: Sense,
        add_content: impl FnOnce(&mut Ui) -> R,
    ) -> Option<Response> {
        if self.cell_count >= self.body.table_layout.columns.len() {
            return None;
        }

        let Column { fixed, layout, .. } =
            self.body.table_layout.columns[self.cell_count].definition;

        let cell_rect = self.get_cell_rect();
        let clip_rect = if fixed {
            constrain_top_bottom(cell_rect, self.body.table_layout.free_viewport)
        } else {
            constrain_to(cell_rect, self.body.table_layout.free_viewport)
        };

        // Draw cell background.
        self.body.ui.painter().rect_filled(
            align_to_pixel(clip_rect, self.body.ui.painter()),
            0.0,
            self.body.ui.style().visuals.window_fill,
        );
        if self.body.striped && self.body.row_count % 2 == 1 {
            self.body.ui.painter().rect_filled(
                align_to_pixel(clip_rect, self.body.ui.painter()),
                0.0,
                self.body.ui.style().visuals.faint_bg_color,
            );
        }

        // Show the cell.
        let ui_rect = cell_rect.expand2(-self.body.ui.spacing().item_spacing);
        let mut child_ui = self.body.ui.child_ui(ui_rect, layout);
        child_ui.set_clip_rect(clip_rect);
        add_content(&mut child_ui);

        let response = self.body.ui.allocate_rect(clip_rect, sense);

        if was_hoverd_strict(&response) {
            self.cell_was_hovered = true;
        }

        let column_layout = &mut self.body.table_layout.columns[self.cell_count];
        if column_layout.definition.is_auto_sized() && column_layout.first_time {
            let content_width = child_ui
                .min_rect()
                .expand2(child_ui.spacing().item_spacing)
                .width()
                + 1.0;
            if content_width > column_layout.content_width {
                column_layout.content_width = content_width;
            }
        }

        self.cell_count += 1;
        Some(Response {
            hovered: was_hoverd_strict(&response),
            ..response
        })
    }

    fn get_cell_rect(&self) -> Rect {
        let column = &self.body.table_layout.columns[self.cell_count];

        let width = if column.definition.is_auto_sized() && column.first_time {
            f32::INFINITY
        } else {
            column.width
        };

        Rect::from_min_size(
            pos2(column.x_pos, self.rect.min.y),
            vec2(width, self.config.height),
        )
    }
}

fn constrain_to(rect: Rect, constraint: Rect) -> Rect {
    Rect::from_min_max(
        rect.min.at_least(constraint.min),
        rect.max.at_most(constraint.max),
    )
}

fn constrain_top_bottom(rect: Rect, constraint: Rect) -> Rect {
    Rect::from_min_max(
        pos2(rect.min.x, rect.min.y.at_least(constraint.min.y)),
        pos2(rect.max.x, rect.max.y.at_most(constraint.max.y)),
    )
}

fn constrain_left_right(rect: Rect, constraint: Rect) -> Rect {
    Rect::from_min_max(
        pos2(rect.min.x.at_least(constraint.min.x), rect.min.y),
        pos2(rect.max.x.at_most(constraint.max.x), rect.max.y),
    )
}

fn align_to_pixel(rect: Rect, painter: &Painter) -> Rect {
    Rect::from_min_max(
        painter.round_pos_to_pixels(rect.min),
        painter.round_pos_to_pixels(rect.max),
    )
}

/// Check if the response was actually hovered by doing a
/// check if the position was inside the rect.
///
/// In the `Sense::hover()` mode egui check for hover inclusively.
/// This checks exclusively.
fn was_hoverd_strict(response: &Response) -> bool {
    response.hover_pos().map_or(false, |pos| {
        pos.x >= response.rect.left()
            && pos.x < response.rect.right()
            && pos.y >= response.rect.top()
            && pos.y < response.rect.bottom()
    })
}
