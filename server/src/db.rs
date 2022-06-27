use serde::{Serialize as se,Deserialize as dse};

#[derive(se)]
pub struct KernelList {
    kernels:Vec<Kernels>,
}
#[derive(se)]

