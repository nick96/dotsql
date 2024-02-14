use std::{collections::HashMap, ops::ControlFlow};

use sqlparser::{
    ast::{visit_statements, Statement, TableConstraint},
    dialect::MySqlDialect,
    parser::Parser,
};

fn main() {
    let schema_file = std::env::args().collect::<Vec<String>>()[1].clone();
    let schema = std::fs::read_to_string(schema_file).unwrap();
    let ast = Parser::parse_sql(&MySqlDialect {}, &schema).unwrap();
    let mut relationships: HashMap<String, Vec<String>> = HashMap::new();
    visit_statements(&ast, |s| {
        if let Statement::CreateTable {
            name,
            columns: _,
            constraints,
            ..
        } = s
        {
            let fks = constraints
                .iter()
                .filter_map(|c| {
                    if let TableConstraint::ForeignKey {
                        name: _,
                        columns: _,
                        foreign_table,
                        referred_columns: _,
                        ..
                    } = c
                    {
                        Some(
                            foreign_table
                                .0
                                .iter()
                                .map(|i| i.value.clone())
                                .collect::<Vec<String>>()
                                .join("."),
                        )
                    } else {
                        None
                    }
                })
                .collect();
            relationships.insert(
                name.0
                    .iter()
                    .map(|i| i.value.clone())
                    .collect::<Vec<String>>()
                    .join(".")
                    .to_string(),
                fks,
            );
        }
        ControlFlow::<()>::Continue(())
    });


    println!("digraph {{");
    for (t, rs) in relationships.iter() {
        for r in rs {
            println!("\t{} -> {}", t, r);
        }
    }
    println!("}}");
}
