use crate::data::{Name, Row, Table};

#[derive(Clone, Copy)]
pub enum SortBy {
    Max,
    Name,
}

pub enum Op {
    Combine(Box<Op>, Box<Op>),
    GroupByCountry,
    Filter(Vec<String>),
    Limit(usize),
    Select {
        start: usize,
        size: usize,
        step: usize,
    },
    SortBy(SortBy),
    GreaterThan(u32),
}

pub fn eval(op: &Op, table: &Table) -> Table {
    match op {
        Op::Filter(names) => filter(table, names),
        Op::GroupByCountry => group_by(table),
        Op::Limit(lmit) => limit(table, *lmit),
        Op::Select { start, size, step } => select(table, *start, *size, *step),
        Op::SortBy(op) => sort_by(table, *op),
        Op::Combine(op1, op2) => eval(op2, &eval(op1, table)),
        Op::GreaterThan(v) => greater_than(table, *v),
    }
}

fn filter(table: &Table, names: &[String]) -> Table {
    let header = table.header.clone();
    let rows = table
        .rows
        .iter()
        .filter(|row| names.iter().any(|name| &row.name.country == name))
        .cloned()
        .collect();
    Table { header, rows }
}

fn select_<'a, T: Clone + 'a>(
    iter: impl Iterator<Item = &'a T>,
    start: usize,
    size: usize,
    step: usize,
) -> Vec<T> {
    iter.skip(start).step_by(step).take(size).cloned().collect()
}

fn select(table: &Table, start: usize, size: usize, step: usize) -> Table {
    let header = std::rc::Rc::new(select_(table.header.iter(), start, size, step));
    let rows: Vec<_> = table
        .rows
        .iter()
        .map(|row| Row {
            name: row.name.clone(),
            data: select_((&row.data).iter(), start, size, step),
        })
        .collect();
    Table { header, rows }
}

fn group_by(table: &Table) -> Table {
    let mut btree = std::collections::BTreeMap::new();
    let header = table.header.clone();
    for row in &table.rows {
        let name = &row.name.country;
        btree
            .entry(name)
            .and_modify(|data: &mut Vec<u32>| {
                data.iter_mut()
                    .zip(&row.data)
                    .for_each(|(val1, val2)| *val1 += *val2)
            })
            .or_insert(row.data.clone());
    }
    let rows = btree
        .into_iter()
        .map(|(country, data)| Row {
            name: Name::new("", country),
            data,
        })
        .collect();
    Table { header, rows }
}

fn sort_by(table: &Table, sort: SortBy) -> Table {
    let header = table.header.clone();
    let mut rows = table.rows.clone();
    match sort {
        SortBy::Name => rows.sort_by_key(|row| row.name.clone()),
        SortBy::Max => rows.sort_by_key(|row| row.data[0]),
    }
    Table { header, rows }
}

fn limit(table: &Table, limit: usize) -> Table {
    let header = table.header.clone();
    let rows = table.rows.iter().rev().take(limit).rev().cloned().collect();
    Table { header, rows }
}

fn greater_than(table: &Table, min: u32) -> Table {
    let header = table.header.clone();
    let rows = table
        .rows
        .clone()
        .into_iter()
        .filter(|row| matches!(row.data.first(), Some(val) if min < *val))
        .collect();
    Table { header, rows }
}
