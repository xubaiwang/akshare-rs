use std::{
    collections::{HashMap, VecDeque},
    fs,
    mem::replace,
    ops::Deref,
};

use nom::{
    bytes::complete::tag,
    character::complete::{one_of, space0},
    IResult, Parser as _,
};
use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, TagEnd};
use replace_with::replace_with_or_abort;

use crate::dirs::data_doc_dir;

/// (stem1, stem2) => ops
pub type Modules = HashMap<String, HashMap<String, Module>>;
pub type Module = HashMap<String, Op>;

/// 讀取兩層目錄所有 Markdown 文件。
///
/// 每個 Markdown 文件映射爲一個 `mod`.
pub fn parse_modules() -> Modules {
    let data_doc_dir = data_doc_dir();

    let mut modules = HashMap::new();

    // 外層
    for entry_outer in fs::read_dir(&data_doc_dir).unwrap() {
        let entry_outer = entry_outer.unwrap();
        let path = entry_outer.path();

        let mut submodules = HashMap::new();

        // 外層只處理文件夾
        if entry_outer.file_type().unwrap().is_dir() {
            // 一級模塊名
            let stem1 = path.file_stem().unwrap().to_str().unwrap();

            // 內層
            for entry_inner in fs::read_dir(&path).unwrap() {
                let path = entry_inner.unwrap().path();

                // 內層只讀取 Markdown 文件
                if matches!(path.extension().and_then(|e| e.to_str()), Some("md")) {
                    // 二級模塊名
                    let stem2 = path.file_stem().unwrap().to_str().unwrap();

                    // 從文件提取信息
                    let text = fs::read_to_string(&path).unwrap();
                    let ops = extract_module(&text);

                    // 內層收集
                    submodules.insert(stem2.to_string(), ops);
                }
            }

            // 外層收集
            modules.insert(stem1.to_string(), submodules);
        }
    }

    modules
}

/// param name => param type
pub type Op = HashMap<String, String>;

pub fn extract_module(text: &str) -> Module {
    // the overall result sink
    let mut module = HashMap::new();

    // handler states
    let mut prev_text: Option<CowStr> = None;
    let mut op_name: Option<CowStr> = None;
    let mut state = ParamTableState::Pre;

    // iterate over parser events
    parser(text).for_each(|event| {
        extract_op_name(&event, &prev_text, &mut op_name);
        extract_param_table(&event, &prev_text, &mut state);
        update_prev_text(&event, &mut prev_text);
        collect_op(&mut module, &mut op_name, &mut state);
    });

    module
}

pub fn parser(text: &str) -> Parser {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    Parser::new_ext(text, options)
}

/// 推入上個文本。
pub fn update_prev_text(event: &Event, prev_text: &mut Option<CowStr>) {
    match event {
        Event::Text(current_text) => {
            _ = prev_text.insert(current_text.clone().into_static());
        }
        _ => {}
    }
}

/// 提取接口名稱。
pub fn extract_op_name(event: &Event, prev_text: &Option<CowStr>, op_name: &mut Option<CowStr>) {
    match event {
        Event::Text(current_text) => {
            // 情況一：行內
            if let Ok((rest, _)) = inline_op_name(&current_text) {
                _ = op_name.insert(CowStr::from(rest).into_static());
            }
            // 情況二：單獨 heading 後
            if let Some(prev_text) = prev_text {
                if prev_text.deref() == "接口名称" {
                    _ = op_name.insert(current_text.clone().into_static());
                }
            }
        }
        _ => {}
    }

    /// 行內接口名稱，析之以 `nom`.
    pub fn inline_op_name<'a>(input: &'a str) -> IResult<&'a str, ()> {
        (tag("接口"), one_of(":："), space0)
            .map(|_| ())
            .parse(input)
    }
}

#[derive(Debug)]
pub enum ParamTableState {
    /// Before entering table.
    Pre,
    /// Enter table => alloc outer
    /// Exit table => convert to finished
    Table(Vec<VecDeque<String>>),
    /// Enter row => alloc inner
    /// Exit row => push inner to outer
    Row(Vec<VecDeque<String>>, VecDeque<String>),
    /// Enter cell => alloc string
    /// Exit cell => push
    Cell(Vec<VecDeque<String>>, VecDeque<String>, String),
    /// After parsing finished, with transformed data.
    /// vec of param name and type
    Finished(Op),
}

/// 當完成表格時返回 `true`.
pub fn extract_param_table(event: &Event, prev_text: &Option<CowStr>, state: &mut ParamTableState) {
    match event {
        Event::Start(tag) => match tag {
            // enter table
            Tag::Table(_) => {
                if let Some(prev_text) = prev_text {
                    if matches!(prev_text.deref(), "输入参数" | "请求参数") {
                        *state = ParamTableState::Table(Vec::new());
                    }
                }
            }
            // enter row
            Tag::TableRow => {
                replace_with_or_abort(state, |state| match state {
                    ParamTableState::Table(outer) => ParamTableState::Row(outer, VecDeque::new()),
                    state => state,
                });
            }
            // enter cell
            Tag::TableCell => {
                replace_with_or_abort(state, |state| match state {
                    ParamTableState::Row(outer, inner) => {
                        ParamTableState::Cell(outer, inner, String::new())
                    }
                    state => state,
                });
            }
            _ => {}
        },
        Event::End(tag_end) => match tag_end {
            // exit table
            TagEnd::Table => {
                replace_with_or_abort(state, |state| match state {
                    ParamTableState::Table(mut outer) => {
                        let mut op = HashMap::new();

                        for row in outer.iter_mut() {
                            if let (Some(name), Some(ty)) = (row.pop_front(), row.pop_front()) {
                                // skip `-` lines
                                if name.as_str() != "-" {
                                    if op.contains_key(&name) {
                                        eprintln!("DUPLICATE PARAM: {}", &name);
                                    }
                                    op.insert(name, ty);
                                }
                            }
                        }
                        ParamTableState::Finished(op)
                    }
                    state => state,
                });
            }
            // exit row
            TagEnd::TableRow => {
                replace_with_or_abort(state, |state| match state {
                    ParamTableState::Row(mut outer, inner) => {
                        outer.push(inner);
                        ParamTableState::Table(outer)
                    }
                    state => state,
                });
            }
            // exit cell
            TagEnd::TableCell => {
                replace_with_or_abort(state, |state| match state {
                    ParamTableState::Cell(outer, mut inner, buf) => {
                        inner.push_back(buf);
                        ParamTableState::Row(outer, inner)
                    }
                    state => state,
                });
            }
            _ => {}
        },
        Event::Text(current_text) => {
            if let ParamTableState::Cell(_, _, buf) = state {
                buf.push_str(&current_text);
            }
        }
        _ => {}
    }
}

pub fn collect_op(module: &mut Module, op_name: &mut Option<CowStr>, state: &mut ParamTableState) {
    if matches!(state, ParamTableState::Finished(_)) {
        // swap reset
        let op_name = op_name.take();
        let state = replace(state, ParamTableState::Pre);

        // collect
        let name = op_name.unwrap().to_string();
        let op = match state {
            ParamTableState::Finished(vec) => vec,
            _ => unreachable!(),
        };

        // show duplicate
        if module.contains_key(&name) {
            eprintln!("DUPLICATE OP: {}", name);
        }
        module.insert(name, op);
    }
}
