use crate::ast::{Expr, ExprX, Exprs, HeaderExprX, Ident, Stmt, StmtX, Typ, VirErr};
use crate::ast_util::err_str;
use crate::def::Spanned;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Header {
    pub hidden: Vec<Ident>,
    pub require: Exprs,
    pub ensure_id_typ: Option<(Ident, Typ)>,
    pub ensure: Exprs,
    pub invariant: Exprs,
}

fn read_header_block(block: &mut Vec<Stmt>) -> Result<Header, VirErr> {
    let mut hidden: Vec<Ident> = Vec::new();
    let mut require: Option<Exprs> = None;
    let mut ensure: Option<(Option<(Ident, Typ)>, Exprs)> = None;
    let mut invariant: Option<Exprs> = None;
    let mut n = 0;
    for stmt in block.iter() {
        match &stmt.x {
            StmtX::Expr(expr) => match &expr.x {
                ExprX::Header(header) => match &**header {
                    HeaderExprX::Requires(es) => {
                        if require.is_some() {
                            return err_str(
                                &stmt.span,
                                "only one call to requires allowed (use requires([e1, ..., en]) for multiple expressions",
                            );
                        }
                        require = Some(es.clone());
                    }
                    HeaderExprX::Ensures(id_typ, es) => {
                        if ensure.is_some() {
                            return err_str(
                                &stmt.span,
                                "only one call to ensures allowed (use ensures([e1, ..., en]) for multiple expressions",
                            );
                        }
                        ensure = Some((id_typ.clone(), es.clone()));
                    }
                    HeaderExprX::Invariant(es) => {
                        if invariant.is_some() {
                            return err_str(
                                &stmt.span,
                                "only one call to invariant allowed (use invariant([e1, ..., en]) for multiple expressions",
                            );
                        }
                        invariant = Some(es.clone());
                    }
                    HeaderExprX::Hide(x) => {
                        hidden.push(x.clone());
                    }
                },
                _ => break,
            },
            _ => break,
        }
        n += 1;
    }
    *block = block[n..].to_vec();
    let require = require.unwrap_or(Rc::new(vec![]));
    let (ensure_id_typ, ensure) = match ensure {
        None => (None, Rc::new(vec![])),
        Some((id_typ, es)) => (id_typ, es),
    };
    let invariant = invariant.unwrap_or(Rc::new(vec![]));
    Ok(Header { hidden, require, ensure_id_typ, ensure, invariant })
}

pub fn read_header(body: &mut Expr) -> Result<Header, VirErr> {
    match &body.x {
        ExprX::Block(stmts, expr) => {
            let mut block: Vec<Stmt> = (**stmts).clone();
            let header = read_header_block(&mut block)?;
            *body = Spanned::new(body.span.clone(), ExprX::Block(Rc::new(block), expr.clone()));
            Ok(header)
        }
        _ => read_header_block(&mut vec![]),
    }
}