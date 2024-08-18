use std::rc::Rc;

use crate::compiler::{
    ds::{ symbol_table::SSAKey, value::{ ops::{ BinaryOp, UnaryOp }, Value } },
    traits::{ DAGNodeTrait, Dissasemble, ParseConnectedNodes },
};

// #[derive(Debug)]
// pub struct DAGFnCallNode {
//     ssa_key: SSAKey,
// }

// impl DAGFnCallNode {
//     pub fn new(ssa_key: SSAKey) -> Self {
//         Self { ssa_key }
//     }

//     pub fn get_ident(&self) -> Rc<str> {
//         self.ssa_key.get_ident()
//     }

//     pub fn get_ssa_key(&self) -> &SSAKey {
//         &self.ssa_key
//     }
// }

// impl ParseConnectedNodes for DAGFnCallNode {
//     type ConnectedNodes = ();

//     fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
//         todo!()
//     }
// }

// impl DAGNodeTrait for DAGFnCallNode {
//     fn expected_connected_nodes(&self) -> usize {
//         todo!()
//     }
// }

// impl Dissasemble for DAGFnCallNode {
//     fn dissasemble(&self) -> String {
//         format!("{}(", self.ssa_key.dissasemble())
//     }
// }
