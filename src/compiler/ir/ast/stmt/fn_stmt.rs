use std::rc::Rc;

use crate::compiler::{
    ds::{ ssa_ident::SSAIdent, symbol_table::UserSymbolVar, value::ValueType },
    error_handler::ErrorHandler,
    ir::{ ast::AST_DISSASEMBLE_INDENTATION, icfg::icfg_builder::{ CFGBuilder, ICFGBuilder } },
    parser::token::TokenMetadata,
    print_todo,
    traits::{ AstDissasemble, Dissasemble, LinearControlFlow, StmtTrait },
    ProgramSymbolTablePhase1,
    SymbolFn,
    SymbolVar,
};

use super::{ BlockStmt, FnArg, GotoNodeIds };

#[derive(Debug)]
pub struct FunctionStmt<'ast> {
    ssa_ident: SSAIdent,
    args: Rc<[FnArg]>,
    body: BlockStmt<'ast>,
    return_type: ValueType,
    ident_metadata: TokenMetadata,
}

impl<'ast> FunctionStmt<'ast> {
    pub fn new(
        ssa_ident: SSAIdent,
        args: Rc<[FnArg]>,
        mut body: BlockStmt<'ast>,
        return_type: ValueType,
        ident_metadata: TokenMetadata
    ) -> Self {
        body.set_is_basic_block(false);
        Self {
            ssa_ident,
            args,
            body,
            return_type,
            ident_metadata,
        }
    }

    pub fn get_ssa_ident(&self) -> &SSAIdent {
        &self.ssa_ident
    }

    pub fn get_mut_ssa_ident(&mut self) -> &mut SSAIdent {
        &mut self.ssa_ident
    }

    pub fn get_args(&self) -> &Rc<[FnArg]> {
        &self.args
    }

    pub fn get_return_type(&self) -> &ValueType {
        &self.return_type
    }

    pub fn get_body(&self) -> &BlockStmt<'ast> {
        &self.body
    }

    pub fn get_ident_metadata(&self) -> TokenMetadata {
        self.ident_metadata
    }
}

impl<'ast> StmtTrait for FunctionStmt<'ast> {
    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        _: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) {
        let mut fn_cfg_builder = CFGBuilder::new(
            self.ssa_ident.clone(),
            self.return_type.clone(),
            self.args.len()
        );

        print_todo("Declare args in new cfg");

        self.body.compile_into_icfg(icfg_builder, &mut fn_cfg_builder, goto_node_ids);

        let cfg_id = icfg_builder.push_cfg(fn_cfg_builder.end_cfg());

        icfg_builder.insert_fn_cfg_id(self.ssa_ident.clone(), cfg_id);
    }

    fn is_linear_control_flow(&self) -> bool {
        false
    }

    fn validate_stmt(
        &mut self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        error_handler: &mut ErrorHandler
    ) {
        // if
        //     let Err(compile_error) = program_symbol_table.insert_fn(
        //         self.ssa_ident.clone(),
        //         SymbolFn::new(self.ident_metadata, self.args.clone(), self.return_type.clone())
        //     )
        // {
        //     error_handler.report_compile_error(compile_error);
        //     // Should not return, because it's only name collisions
        // }

        let prev_symbol_table_id = program_symbol_table.get_current_symbol_table_id();
        program_symbol_table.set_current_symbol_table_id(prev_symbol_table_id + 1);

        for arg in self.args.iter() {
            program_symbol_table.insert_var(
                self.ssa_ident.clone(),
                UserSymbolVar::new(
                    arg.value_type.clone(),
                    arg.ident_metadata,
                    arg.mut_keyword_metadata
                )
            );
        }

        self.body.validate_stmt(program_symbol_table, error_handler);

        program_symbol_table.set_current_symbol_table_id(prev_symbol_table_id);
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        None
    }
}

impl<'ast> Dissasemble for FunctionStmt<'ast> {
    fn dissasemble(&self) -> String {
        let mut string_builder = format!("fn {}(", self.ssa_ident.dissasemble());
        for i in 0..self.args.len() {
            string_builder += &self.args[i].ssa_ident.dissasemble();
            string_builder += " ";
            string_builder += &self.args[i].value_type.dissasemble();
            if i != 0 {
                string_builder += ", ";
            }
        }
        string_builder += format!(") {}", if self.return_type != ValueType::Void {
            self.return_type.dissasemble() + " "
        } else {
            "".to_string()
        }).as_str();

        string_builder += "{\n";
        string_builder += self.body.dissasemble().as_str();
        string_builder += "}\n";

        string_builder
    }
}

impl<'ast> AstDissasemble for FunctionStmt<'ast> {
    fn ast_dissasemble(
        &self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        scope_depth: usize
    ) -> String {
        let mut string_builder = format!(
            "[{}]: {}fn {}(",
            program_symbol_table.get_current_symbol_table_id(),
            " ".repeat(AST_DISSASEMBLE_INDENTATION * scope_depth),
            self.ssa_ident.dissasemble()
        );
        for i in 0..self.args.len() {
            string_builder += &self.args[i].ssa_ident.dissasemble();
            string_builder += " ";
            string_builder += &self.args[i].value_type.dissasemble();
            if i != 0 {
                string_builder += ", ";
            }
        }
        string_builder += format!(") {}", if self.return_type != ValueType::Void {
            self.return_type.dissasemble() + " "
        } else {
            "".to_string()
        }).as_str();

        string_builder += "{\n";
        string_builder += self.body.ast_dissasemble(program_symbol_table, scope_depth).as_str();
        string_builder += format!(
            "[{}]: }}\n",
            program_symbol_table.get_current_symbol_table_id()
        ).as_str();

        string_builder
    }
}
