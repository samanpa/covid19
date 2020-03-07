use crate::data::{Place, Row, Table};

pub enum Op {
    //GroupBy{ name: String, place: Place },
    Filter {
        name: String,
        place: Place,
    },
    Select {
        start: usize,
        size: usize,
        step: usize,
    },
    Combine(Box<Op>, Box<Op>),
}

pub fn eval(op: &Op, table: &Table) -> Table {
    match op {
        Op::Filter { name, place } => filter(table, name, *place),
        Op::Select { start, size, step } => select(table, *start, *size, *step),
        Op::Combine(op1, op2) => eval(op2, &eval(op1, table)),
    }
}

fn filter(table: &Table, name: &str, place: Place) -> Table {
    let header = table.header.clone();
    let rows = table
        .rows
        .iter()
        .filter(|row| row.name.get(place) == name)
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
