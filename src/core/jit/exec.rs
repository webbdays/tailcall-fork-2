use std::fmt::Debug;
use std::mem;
use std::sync::{Arc, Mutex};

use async_graphql::Positioned;
use derive_getters::Getters;
use futures_util::future::join_all;

use super::context::Context;
use super::{DataPath, OperationPlan, Request, Response, Store};
use crate::core::ir::model::IR;
use crate::core::jit;
use crate::core::jit::synth::Synth;
use crate::core::json::{JsonLike, JsonObjectLike};

type SharedStore<Output, Error> = Arc<Mutex<Store<Result<Output, Positioned<Error>>>>>;

///
/// Default GraphQL executor that takes in a GraphQL Request and produces a
/// GraphQL Response
pub struct Executor<IRExec, Input> {
    plan: OperationPlan<Input>,
    exec: IRExec,
}

impl<Input, Output, Exec> Executor<Exec, Input>
where
    Output:
        for<'b> JsonLike<'b, JsonObject<'b>: JsonObjectLike<'b, Value = Output>> + Debug + Clone,
    Input: Clone + Debug,
    Exec: IRExecutor<Input = Input, Output = Output, Error = jit::Error>,
{
    pub fn new(plan: OperationPlan<Input>, exec: Exec) -> Self {
        Self { plan, exec }
    }

    pub async fn store(
        &self,
        request: Request<Input>,
    ) -> Store<Result<Output, Positioned<jit::Error>>> {
        let store = Arc::new(Mutex::new(Store::new()));
        let mut ctx = ExecutorInner::new(request, store.clone(), self.plan.to_owned(), &self.exec);
        ctx.init().await;

        let store = mem::replace(&mut *store.lock().unwrap(), Store::new());
        store
    }

    pub async fn execute(self, synth: Synth<Output>) -> Response<Output, jit::Error> {
        Response::new(synth.synthesize())
    }
}

#[derive(Getters)]
struct ExecutorInner<'a, Input, Output, Error, Exec> {
    request: Request<Input>,
    store: SharedStore<Output, Error>,
    plan: OperationPlan<Input>,
    ir_exec: &'a Exec,
}

impl<'a, Input, Output, Error, Exec> ExecutorInner<'a, Input, Output, Error, Exec>
where
    Output: for<'i> JsonLike<'i> + Debug,
    Input: Clone + Debug,
    Exec: IRExecutor<Input = Input, Output = Output, Error = Error>,
{
    fn new(
        request: Request<Input>,
        store: SharedStore<Output, Error>,
        plan: OperationPlan<Input>,
        ir_exec: &'a Exec,
    ) -> Self {
        Self { request, store, plan, ir_exec }
    }

    async fn init(&mut self) {
        join_all(self.plan.as_nested().iter().map(|field| async {
            let mut arg_map = indexmap::IndexMap::new();
            for arg in field.args.iter() {
                let name = arg.name.as_str();
                let value: Option<Input> = arg
                    .value
                    .clone()
                    // TODO: default value resolution should happen in the InputResolver
                    .or_else(|| arg.default_value.clone());

                if let Some(value) = value {
                    arg_map.insert(name, value);
                } else if !arg.type_of.is_nullable() {
                    // TODO: throw error here
                    todo!()
                }
            }
            // TODO: with_args should be called on inside iter_field on any level, not only
            // for root fields
            let ctx = Context::new(&self.request, self.plan.is_query(), field).with_args(arg_map);
            self.execute(&ctx, DataPath::new()).await
        }))
        .await;
    }

    async fn iter_field<'b>(
        &'b self,
        ctx: &'b Context<'b, Input, Output>,
        data_path: &DataPath,
        value: &'b Output,
    ) -> Result<(), Error> {
        let field = ctx.field();
        // Array
        // Check if the field expects a list
        if field.type_of.is_list() {
            // Check if the value is an array
            if let Some(array) = value.as_array() {
                join_all(field.nested_iter().map(|field| {
                    join_all(array.iter().enumerate().map(|(index, value)| {
                        let ctx = ctx.with_value_and_field(value, field);
                        let data_path = data_path.clone().with_index(index);
                        async move { self.execute(&ctx, data_path).await }
                    }))
                }))
                .await;
            }
            // TODO:  We should throw an error stating that we expected
            // a list type here but because the `Error` is a
            // type-parameter, its not possible
        }
        // TODO: Validate if the value is an Object
        // Has to be an Object, we don't do anything while executing if its a Scalar
        else {
            join_all(field.nested_iter().map(|child| {
                let ctx = ctx.with_value_and_field(value, child);
                let data_path = data_path.clone();
                async move { self.execute(&ctx, data_path).await }
            }))
            .await;
        }

        Ok(())
    }

    async fn execute<'b>(
        &'b self,
        ctx: &'b Context<'b, Input, Output>,
        data_path: DataPath,
    ) -> Result<(), Error> {
        let field = ctx.field();

        if let Some(ir) = &field.ir {
            let result = self.ir_exec.execute(ir, ctx).await;

            if let Ok(ref value) = result {
                self.iter_field(ctx, &data_path, value).await?;
            }

            let mut store = self.store.lock().unwrap();

            store.set(
                &field.id,
                &data_path,
                result.map_err(|e| Positioned::new(e, field.pos)),
            );
        } else {
            // if the present field doesn't have IR, still go through it's extensions to see
            // if they've IR.
            let default_obj = Output::object(Output::JsonObject::new());
            let value = ctx
                .value()
                .and_then(|v| v.get_key(&field.name))
                // in case there is no value we still put some dumb empty value anyway
                // to force execution of the nested fields even when parent object is not present.
                // For async_graphql it's done by `fix_dangling_resolvers` fn that basically creates
                // fake IR that resolves to empty object. The `fix_dangling_resolvers` is also
                // working here, but eventually it can be replaced by this logic
                // here without doing the "fix"
                .unwrap_or(&default_obj);

            self.iter_field(ctx, &data_path, value).await?;
        }

        Ok(())
    }
}

/// Executor for IR
#[async_trait::async_trait]
pub trait IRExecutor {
    type Input;
    type Output;
    type Error;
    async fn execute<'a>(
        &'a self,
        ir: &'a IR,
        ctx: &'a Context<'a, Self::Input, Self::Output>,
    ) -> Result<Self::Output, Self::Error>;
}
