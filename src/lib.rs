use slint::SharedString;
use slint::VecModel;
use std::rc::Rc;

pub fn calculate_cost_est(cost: f32, time_span: SharedString) -> (f32, f32, f32, f32) {
    let time_span_str: &str = &time_span;
    let (daily, weekly, monthly, yearly) = match time_span_str {
        "Daily" => (cost, cost * 7.0, cost * 30.0, cost * 365.0),
        "Weekly" => (cost / 7.0, cost, cost * 4.2857, cost * 52.1429),
        "Monthly" => (cost / 30.0, cost / 4.2857, cost, cost * 12.0),
        "Yearly" => (cost / 365.0, cost / 52.1429, cost / 12.0, cost),
        _ => (0.0, 0.0, 0.0, 0.0), // Handle unexpected values
    };

    (
        (daily * 100.0).round() / 100.0,
        (weekly * 100.0).round() / 100.0,
        (monthly * 100.0).round() / 100.0,
        (yearly * 100.0).round() / 100.0,
    )
}

#[derive(Debug, Clone)]
pub struct Row {
    name: SharedString,
    cost: f32,
    time_span: SharedString,
}

pub fn add_new_item(item_list: &Rc<VecModel<Row>>, item_name: SharedString, value: f32, time_span_val: SharedString) {
    item_list.push(Row {
        name: item_name,
        cost: value,
        time_span: time_span_val,
    });
}

pub fn remove_item(item_list: &Rc<VecModel<Row>>, index: usize) {
    item_list.remove(index);
}