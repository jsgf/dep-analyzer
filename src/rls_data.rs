use std::path::PathBuf;

use serde::Deserializer;
use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub struct Analysis {
    pub config: Config,
    pub version: String,
    pub compilation: Compilation,
    pub prelude: Prelude,
    pub imports: Vec<Import>,
    pub defs: Vec<Def>,
    pub impls: Vec<Impl>,
    pub refs: Vec<Ref>,
    pub macro_refs: Vec<MacroRef>,
    pub relations: Vec<Relation>,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub struct Config {
    pub output_file: Option<PathBuf>,
    pub full_docs: bool,
    pub pub_only: bool,
    pub reachable_only: bool,
    pub distro_crate: bool,
    pub signatures: bool,
    pub borrow_data: bool,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub struct Compilation {
    #[serde(deserialize_with = "pathbuf_deserialize")]
    pub directory: PathBuf,
    pub program: PathBuf,
    pub arguments: Vec<String>,
    #[serde(deserialize_with = "pathbuf_deserialize")]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub struct Prelude {
    pub crate_id: GlobalCrateId,
    pub external_crates: Vec<ExternalCrate>,
    pub span: Span,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub struct ExternalCrate {
    pub file_name: PathBuf,
    pub num: u32,
    pub id: GlobalCrateId,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub struct GlobalCrateId {
    pub name: String,
    pub disambiguator: (u64, u64),
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub struct Span {
    #[serde(deserialize_with = "pathbuf_deserialize")]
    pub file_name: PathBuf,
    pub byte_start: u32,
    pub byte_end: u32,
    pub line_start: u32,
    pub line_end: u32,
    pub column_start: u32,
    pub column_end: u32,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub struct Import {
    pub kind: ImportKind,
    pub ref_id: Option<Id>,
    pub span: Span,
    pub alias_span: Option<Span>,
    pub name: String,
    pub value: String,
    pub parent: Option<Id>,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub enum ImportKind {
    ExternCrate,
    Use,
    GlobUse,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub struct Def {
    pub kind: DefKind,
    pub id: Id,
    pub span: Span,
    pub name: String,
    pub qualname: String,
    pub value: String,
    pub parent: Option<Id>,
    pub children: Vec<Id>,
    pub decl_id: Option<Id>,
    pub docs: String,
    pub sig: Option<Signature>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub enum DefKind {
    Enum,
    TupleVariant,
    StructVariant,
    Tuple,
    Struct,
    Union,
    Trait,
    Function,
    ForeignFunction,
    Method,
    Macro,
    Mod,
    Type,
    Local,
    Static,
    ForeignStatic,
    Const,
    Field,
    ExternType,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub struct Id {
    pub krate: u32,
    pub index: u32,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub struct Signature {
    pub text: String,
    pub defs: Vec<SigElement>,
    pub refs: Vec<SigElement>,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub struct SigElement {
    pub id: Id,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub struct Attribute {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub struct Impl {
    id: u32,
    kind: ImplKind,
    span: Span,
    value: String,
    parent: Option<Id>,
    children: Vec<Id>,
    docs: String,
    sig: Option<Signature>,
    attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub enum ImplKind {
    Inherent,
    Direct,
    Indirect,
    Blanket,
    Deref(String, Id),
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub struct Ref {
    pub kind: RefKind,
    pub span: Span,
    pub ref_id: Id,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub enum RefKind {
    Function,
    Mod,
    Type,
    Variable,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub struct MacroRef {
    pub span: Span,
    pub qualname: String,
    pub callee_span: Span,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
pub struct Relation {
    pub span: Span,
    #[serde(skip)]
    pub kind: Option<RelationKind>,
    pub from: Id,
    pub to: Id,
}

#[derive(Debug, Clone, Deserialize, Hash, Eq, PartialEq)]
//#[serde(tag = "variant", content = "fields")]
pub enum RelationKind {
    Impl { id: u32 },
    SuperTrait,
}

fn pathbuf_deserialize<'de, D>(input: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    use std::os::unix::ffi::OsStringExt;

    let v: Vec<u8> = serde::Deserialize::deserialize(input)?;
    let os = std::ffi::OsString::from_vec(v);
    Ok(PathBuf::from(os))
}
