//! Include —— 多文件加载与合并。
//!
//! 由 CLI/pipeline 在词法/语法 → semantic 之前调用：
//! 顺着 includes 递归读取并 parse 子文件，把它们的 tokens / components 合并到主 program。
//!
//! 主文件之外的 composition 被忽略（include 仅用于复用 token/component）。

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::ast::Program;
use crate::error::{ErrorKind, LottieError, Result};
use crate::lexer::Lexer;
use crate::parser::Parser;

/// 递归加载 include 链，把所有 token / component 合并到主 program。
///
/// `base_dir` 是主文件所在目录，include 路径相对于此。
pub fn load_includes(program: &mut Program, base_dir: &Path) -> Result<()> {
    let mut visited: HashSet<PathBuf> = HashSet::new();
    let main_includes = std::mem::take(&mut program.includes);
    for inc in &main_includes {
        load_one(program, base_dir, &inc.path, &mut visited)?;
    }
    Ok(())
}

fn load_one(
    program: &mut Program,
    base_dir: &Path,
    rel_path: &str,
    visited: &mut HashSet<PathBuf>,
) -> Result<()> {
    let path = base_dir.join(rel_path);
    let canonical = path.canonicalize().map_err(|_| {
        LottieError::new(
            ErrorKind::IncludeFileNotFound,
            format!("找不到 include 文件 `{}`", path.display()),
            None,
        )
    })?;
    if !visited.insert(canonical.clone()) {
        return Ok(()); // 已加载过，跳过
    }

    let source = std::fs::read_to_string(&canonical).map_err(|e| {
        LottieError::new(
            ErrorKind::IncludeFileNotFound,
            format!("读取 {} 失败：{e}", canonical.display()),
            None,
        )
    })?;

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let included = parser.parse()?;

    // 合并 tokens / components；重名报错避免静默 shadow。
    for tg in included.tokens {
        if program.tokens.iter().any(|t| t.name == tg.name) {
            return Err(LottieError::new(
                ErrorKind::DuplicateDefinition,
                format!(
                    "include `{}` 中的 token 组 `{}` 与已加载的同名 token 组冲突",
                    rel_path, tg.name
                ),
                Some(tg.span),
            ));
        }
        program.tokens.push(tg);
    }
    for c in included.components {
        if program.components.iter().any(|x| x.name == c.name) {
            return Err(LottieError::new(
                ErrorKind::DuplicateDefinition,
                format!(
                    "include `{}` 中的 component `{}` 与已加载的同名 component 冲突",
                    rel_path, c.name
                ),
                Some(c.span),
            ));
        }
        program.components.push(c);
    }

    // 递归处理子文件的 includes
    let next_base = canonical
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| base_dir.to_path_buf());
    for inc in &included.includes {
        load_one(program, &next_base, &inc.path, visited)?;
    }

    Ok(())
}

#[cfg(test)]
#[path = "resolve_include_tests.rs"]
mod tests;
