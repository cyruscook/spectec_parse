extern crate proc_macro;

mod atom;
mod node;
mod utils;

use synstructure::decl_derive;

decl_derive!([SpecTecAtom, attributes(spectec_atom)] => atom::spectec_atom_derive);
decl_derive!([SpecTecNode, attributes(spectec_node, spectec_field)] => node::spectec_node_derive);
