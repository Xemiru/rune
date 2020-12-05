use crate::compiling::assemble::prelude::*;

/// Compile an expr field access, like `<value>.<field>`.
impl Assemble for ast::ExprFieldAccess {
    fn assemble(&self, c: &mut Compiler<'_>, needs: Needs) -> CompileResult<Value> {
        let span = self.span();

        // Optimizations!
        //
        // TODO: perform deferred compilation for expressions instead, so we can
        // e.g. inspect if it compiles down to a local access instead of
        // climbing the ast like we do here.
        #[allow(clippy::single_match)]
        match (&self.expr, &self.expr_field) {
            (ast::Expr::Path(path), ast::ExprField::LitNumber(n)) => {
                if let Some(value) =
                    try_immediate_field_access_optimization(c, span, path, n, needs)?
                {
                    return Ok(value);
                }
            }
            _ => (),
        }

        let target = self.expr.assemble(c, Needs::Value)?.consuming_address(c)?;

        // This loop is actually useful.
        #[allow(clippy::never_loop)]
        loop {
            match &self.expr_field {
                ast::ExprField::LitNumber(n) => {
                    let index = match n.resolve(&c.storage, &*c.source)?.as_tuple_index() {
                        Some(n) => n,
                        _ => break,
                    };

                    c.asm.push(Inst::TupleIndexGet { target, index }, span);

                    if !needs.value() {
                        c.warnings.not_used(c.source_id, span, c.context());
                        c.asm.push(Inst::Pop, span);
                        return Ok(Value::empty(span));
                    }

                    return Ok(Value::unnamed(span, c));
                }
                ast::ExprField::Ident(ident) => {
                    let field = ident.resolve(&c.storage, &*c.source)?;
                    let slot = c.unit.new_static_string(span, field.as_ref())?;

                    c.asm.push(Inst::ObjectIndexGet { target, slot }, span);

                    if !needs.value() {
                        c.warnings.not_used(c.source_id, span, c.context());
                        c.asm.push(Inst::Pop, span);
                        return Ok(Value::empty(span));
                    }

                    return Ok(Value::unnamed(span, c));
                }
            }
        }

        Err(CompileError::new(span, CompileErrorKind::BadFieldAccess))
    }
}

fn try_immediate_field_access_optimization(
    this: &mut Compiler<'_>,
    span: Span,
    path: &ast::Path,
    n: &ast::LitNumber,
    needs: Needs,
) -> CompileResult<Option<Value>> {
    let ident = match path.try_as_ident() {
        Some(ident) => ident,
        None => return Ok(None),
    };

    let ident = ident.resolve(this.storage, &*this.source)?;

    let index = match n.resolve(this.storage, &*this.source)? {
        ast::Number::Integer(n) => n,
        _ => return Ok(None),
    };

    let index = match usize::try_from(index) {
        Ok(index) => index,
        Err(..) => return Ok(None),
    };

    let (id, var) =
        match this
            .scopes
            .try_get_var(ident.as_ref(), this.source_id, this.visitor, path.span())?
        {
            Some(var) => var,
            None => return Ok(None),
        };

    this.asm.push(
        Inst::TupleIndexGet {
            target: InstAddress::Offset(var.offset.translate(id)),
            index,
        },
        span,
    );

    if !needs.value() {
        this.warnings.not_used(this.source_id, span, this.context());
        this.asm.push(Inst::Pop, span);
    }

    Ok(Some(Value::unnamed(span, this)))
}
