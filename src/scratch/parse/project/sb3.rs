use std::collections::HashMap;
use zip::read::ZipFile;

pub struct Vec2<T>(T, T);

pub struct AssetId {
    // TODO
}

pub enum ImageDataFormat {
    PNG,
    SVG,
    JPEG,
    JPG,
    BMP,
    GIF,
}

pub enum AudioDataFormat {
    WAV,
    WAVE,
    MP3,
}

pub struct Asset<'a, DataFormat> {
    pub asset_id: AssetId,
    pub data_format: DataFormat,
    pub name: String,
    pub file: ZipFile<'a>,
}

pub struct Costume<'a> {
    pub asset: Asset<'a, ImageDataFormat>,
    pub bitmap_resolution: u32,
    pub rotation_center: Vec2<f64>,
}

pub struct Sound<'a> {
    pub asset: Asset<'a, AudioDataFormat>,
    pub rate: u32,
    pub sample_count: u32,
}

pub enum Number {
    UInt(u64),
    Int(i64),
    Float(f64),
}

pub enum NumberOrString {
    Number(Number),
    String(String),
}

pub enum ScalarValue {
    Bool(bool),
    Number(Number),
    String(String),
}

pub struct Broadcast {
    pub name: String,
}

pub enum Value {
    Scalar(ScalarValue),
    List(Vec<Value>),
    Broadcast(Broadcast),
}

pub struct Variable {
    pub name: String,
    pub value: Value,
    pub on_cloud: bool,
}

pub struct Color {
    // TODO
}

pub type NumPrimitive = NumberOrString;

pub type ColorPrimitive = Color;

pub type TextPrimitive = String;

pub enum VariableType {
    Scalar,
    List,
    Broadcast, // same as Value
}

pub struct VariablePrimitive {
    name: String,
    r#type: VariableType,
    id: String,
    position: Vec2<f64>,
}

pub struct IndexPrimitive {
    value: i32,
}

//#[serde(tag = "kind")]
pub enum Primitive {
    Num(NumPrimitive),
    Color(ColorPrimitive),
    Text(TextPrimitive),
    Variable(VariablePrimitive),
    Index(IndexPrimitive),
}

pub enum BlockCategory {
    Motion,
    Look,
    Sound,
    Event,
    Control,
    Sensing,
    Operator,
    Variable,
    Block,
}

pub enum CategoryOpCode {}

pub struct OpCode {
    pub category: BlockCategory,
    pub op_code: CategoryOpCode,
}

pub enum Shadow {
    UnObscured,
    None,
    Obscured,
}

pub struct Input {
    pub shadow: Shadow,
    pub args: Vec<Primitive>,
}

pub struct Field {
    // TODO
}

pub struct Block<'a> {
    pub op_code: OpCode,
    pub next: Option<&'a Block<'a>>,
    pub parent: Option<&'a Block<'a>>,
    pub comment: Option<String>,
    pub inputs: HashMap<String, Input>,
    pub fields: HashMap<String, Field>,
    pub top_level: bool,
    pub shadow: bool,
    pub position: Vec2<f64>,
}

pub struct Comment<'a> {
    pub block: Option<&'a Block<'a>>,
    pub text: String,
    pub minimized: bool,
    pub position: Vec2<f64>,
}

pub struct Target<'a> {
    pub current_costume: &'a Costume<'a>,
    pub blocks: Vec<Block<'a>>,
    pub variables: Vec<Variable>,
    pub comments: Vec<Comment<'a>>,
    pub costumes: Vec<Costume<'a>>,
    pub sounds: Vec<Sound<'a>>,
    pub volume: f64,
}

pub enum VideoState {
    On,
    Off,
    OnFlipped,
}

pub struct Stage<'a> {
    pub target: Target<'a>,
    pub tempo: f64,
    pub video_transparency: f64,
    pub video_state: VideoState,
}

pub enum RotationStyle {
    AllAround,
    DontRotate,
    LeftRight,
}

pub struct Sprite<'a> {
    pub target: Target<'a>,
    pub name: String,
    pub visible: bool,
    pub position: Vec2<f64>,
    pub size: f64,
    pub direction: f64,
    pub draggable: bool,
    pub rotation_style: RotationStyle,
    pub layer_order: u32,
}

pub struct SemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: String,
}

pub struct Meta {
    pub version: SemVer,
    pub vm: SemVer,
    pub user_agent: String,
}

pub struct Targets<'a> {
    pub stage: Stage<'a>,
    pub sprites: Vec<Sprite<'a>>,
}

pub struct Project<'a> {
    pub targets: Targets<'a>,
    pub meta: Meta,
}
