extern crate proc_macro;

mod node;
mod utils;

use synstructure::decl_derive;

decl_derive!([SpecTecNode, attributes(spectec_node, spectec_field)] => node::spectec_node_derive);
