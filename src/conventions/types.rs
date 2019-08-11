#[derive(Clone, Debug, PartialEq)]
pub enum ValType {
    I32(Option<i32>),
    I64(Option<i64>),
    F32(Option<f32>),
    F64(Option<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FuncType {
    params: Vec<ValType>,
    results: Vec<ValType>,
}

impl FuncType {
    pub fn new(params: Vec<ValType>, results: Vec<ValType>) -> FuncType {
        FuncType { params, results }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Limits {
    min: u32,
    max: Option<u32>,
}

impl Limits {
    pub fn new(min: u32, max: Option<u32>) -> Limits {
        Limits { min, max }
    }
}

#[derive(Clone, Debug)]
pub struct MemType {
    limits: Limits,
}

impl MemType {
    pub fn new(limits: Limits) -> MemType {
        MemType { limits }
    }
}

#[derive(Clone, Debug)]
pub enum ElemType {
    FuncType,
}

#[derive(Clone, Debug)]
pub struct TableType {
    elem_type: ElemType,
    limit: Limits,
}

impl TableType {
    pub fn new(elem_type: ElemType, limit: Limits) -> TableType {
        TableType { elem_type, limit }
    }
}

#[derive(Clone, Debug)]
pub struct GlobalType {
    valtype: ValType,
    r#mut: bool,
}

impl GlobalType {
    pub fn new(valtype: ValType, r#mut: bool) -> GlobalType {
        GlobalType { valtype, r#mut }
    }
}

#[derive(Clone, Debug)]
pub enum ImportDesc {
    Func(FuncType),
    Table(TableType),
    Memory(MemType),
    Global(GlobalType),
}

#[derive(Clone, Debug)]
pub struct Import {
    r#mod: String,
    namespace: String,
    import_desc: ImportDesc,
}

#[derive(Clone, Debug)]
pub enum ExportDesc {
    Func(FuncType),
    Table(TableType),
    Memory(MemType),
    Global(GlobalType),
}

#[derive(Clone, Debug)]
pub struct Export {
    name: String,
    export_desc: ExportDesc,
}

#[derive(Clone, Debug)]
pub struct Elem {
    table_index: u32,
    offset_expr: Vec<u8>,
    func_index: Vec<u8>,
}
#[derive(Clone, Debug)]
pub struct Code {
    size: u32,
    code: Func,
}

#[derive(Clone, Debug)]
pub struct Func {
    locals: Vec<Local>,
    expr: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct Local {
    num_locals: u32,
    r#type: ValType,
}

#[derive(Clone, Debug)]
pub struct Data {
    mem_index: u32,
    offset_expr: Vec<u8>,
    init: Vec<u8>,
}

#[derive(Clone, Debug)]
pub enum SectionBody {
    TypeSection(Vec<FuncType>),
    ImportSection(Import),
    FunctionSection(Vec<FuncType>),
    TableSection(Vec<TableType>),
    MemorySection(Vec<MemType>),
    GlobalSection(Vec<GlobalType>),
    ExportSection(Vec<Export>),
    StartSection(u32),
    ElementSection(Vec<Elem>),
    CodeSection(Vec<Code>),
    DataSection(Vec<Data>),
}
