use std::rc::Rc;

use crate::compiler::{
    ds::{ symbol_table::SymbolTableRef, value::ValueType },
    error_handler::{ CompileError, ErrorHandler, ReportedError, SrcCharsRange },
    ir::{ ast::expr::ExprTrait, icfg::dag::DAG },
    parser::token::TokenMetadata,
    Dissasemble,
};

use super::{ ExprStmt, LinearControlFlow, StmtTrait };

#[derive(Debug)]
pub struct VarDefStmt<'ast> {
    name: Rc<str>,
    value_type: Option<ValueType>,
    is_mutable: bool,
    value: Option<ExprStmt<'ast>>,
    token_metadata: TokenMetadata,
}

impl<'ast> VarDefStmt<'ast> {
    pub fn new(
        name: Rc<str>,
        value_type: Option<ValueType>,
        is_mutable: bool,
        value: Option<ExprStmt<'ast>>,
        token_metadata: TokenMetadata
    ) -> Self {
        Self {
            name,
            value_type,
            is_mutable,
            value,
            token_metadata,
        }
    }

    pub fn get_resolved_value_type(
        &mut self,
        symbol_table_ref: &SymbolTableRef
    ) -> Result<ValueType, CompileError> {
        let provided_value_type = self.value_type.as_ref();
        let value_type_based_on_value = match &mut self.value {
            Some(v) => Some(v.type_check(&symbol_table_ref)?),
            None => None,
        };

        let value_type = match (provided_value_type.cloned(), value_type_based_on_value) {
            (None, None) => {
                return Err(
                    CompileError::new(
                        ReportedError::new(
                            format!(
                                "The type of '{}' cannot be determined. Please provide a type: '{} ..'",
                                self.name,
                                self.name
                            ),
                            self.token_metadata.into()
                        )
                    )
                );
            }
            (Some(provided_type), None) => provided_type,
            (None, Some(found_type)) => found_type,
            (Some(provided_type), Some(found_type)) => {
                if provided_type.is(&found_type) {
                    provided_type
                } else {
                    return Err(
                        CompileError::new(
                            ReportedError::new(
                                format!(
                                    "Type of '{}' is provided as '{}', but type '{}' was found based on its provided value",
                                    self.name,
                                    provided_type.dissasemble(),
                                    found_type.dissasemble()
                                ),
                                {
                                    let mut chars_range: SrcCharsRange = self.token_metadata.into();
                                    chars_range.merge(
                                        &self.value.as_ref().unwrap().collect_metadata()
                                    );
                                    chars_range
                                }
                            )
                        )
                    );
                }
            }
        };
        Ok(value_type)
    }

    pub fn get_name(&self) -> Rc<str> {
        Rc::clone(&self.name)
    }

    pub fn get_value_type(&self) -> Option<&ValueType> {
        self.value_type.as_ref()
    }

    pub fn get_is_mutable(&self) -> bool {
        self.is_mutable
    }

    pub fn get_value(&self) -> Option<&ExprStmt<'ast>> {
        self.value.as_ref()
    }

    pub fn get_metadata(&self) -> TokenMetadata {
        self.token_metadata
    }
}

impl<'ast> Dissasemble for VarDefStmt<'ast> {
    fn dissasemble(&self) -> String {
        let mutable_string = match self.is_mutable {
            true => "mut ".to_string(),
            false => "".to_string(),
        };

        let value_string = match &self.value {
            Some(value) => format!(" := {}", value.dissasemble()),
            None => "".to_string(),
        };

        match &self.value_type {
            Some(value_type) => {
                format!(
                    "{}{}{}{}\n",
                    mutable_string,
                    self.name,
                    value_type.dissasemble(),
                    value_string
                )
            }
            None => { format!("{}{}{}\n", mutable_string, self.name, value_string) }
        }
    }
}

impl<'ast> StmtTrait for VarDefStmt<'ast> {
    fn is_linear_control_flow(&self) -> bool {
        true
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        match symbol_table_ref.get_mut().declare_var(self) {
            Ok(_) => {}
            Err(err) => error_handler.report_compile_error(err),
        }
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        Some(self as &dyn LinearControlFlow)
    }
}

impl<'ast> LinearControlFlow for VarDefStmt<'ast> {
    fn compile_into_dag(&self, dag: &mut DAG) {
        todo!()
    }
}
