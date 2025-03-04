use std::{
    fs,
    io::{self, Write},
};

use crate::{dirs::ops_file_path, parse::Modules};

pub fn dump(modules: Modules) -> io::Result<()> {
    let mut buf = Vec::new();
    for (stem1, submodule) in modules {
        writeln!(buf, "pub mod {} {{", escape(&stem1))?;
        for (stem2, ops) in submodule {
            writeln!(buf, "    pub mod {} {{", escape(&stem2))?;
            writeln!(buf, "        use akshare_macros::define_op;")?;
            for (op_name, params) in ops {
                writeln!(buf, "        define_op! {{")?;
                writeln!(buf, "            name: {},", escape(&op_name))?;
                writeln!(buf, "            params: {{")?;
                for (param_name, param_ty) in params {
                    writeln!(
                        buf,
                        "                {}: {},",
                        escape(&param_name),
                        format_type(&param_ty)
                    )?;
                }
                writeln!(buf, "            }},")?;
                writeln!(buf, "        }}")?;
            }
            writeln!(buf, "    }}")?;
        }
        writeln!(buf, "}}")?;
    }

    fs::write(ops_file_path(), buf)?;

    Ok(())
}

fn escape(ident: &str) -> String {
    match ident {
        "macro" => "r#macro".to_string(),
        _ => ident.to_string(),
    }
}

fn format_type(ty: &str) -> String {
    match ty {
        x @ ("int" | "float" | "str") => x.to_string(),
        x => {
            eprintln!("MAL TYPE: {}", x);
            "str".to_string()
        }
    }
}
