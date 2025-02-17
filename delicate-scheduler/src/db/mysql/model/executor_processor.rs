use super::prelude::*;
use super::schema::executor_processor;

#[derive(Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name = "executor_processor"]

pub struct ExecutorProcessor {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) host: String,
    pub(crate) machine_id: i16,
    description: String,
    tag: String,
    status: i16,
    token: String,
    created_time: NaiveDateTime,
    deleted_time: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "executor_processor"]
pub struct NewExecutorProcessor {
    name: String,
    description: String,
    host: String,
    machine_id: i16,
    tag: String,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Default, Clone, Serialize, Deserialize)]
#[table_name = "executor_processor"]

pub struct UpdateExecutorProcessor {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) host: String,
    pub(crate) machine_id: i16,
    pub(crate) tag: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct QueryParamsExecutorProcessor {
    id: Option<i64>,
    name: Option<String>,
    description: Option<String>,
    host: Option<String>,
    tag: Option<String>,
    machine_id: Option<i16>,
    pub(crate) per_page: i64,
    pub(crate) page: i64,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]

pub struct ExecutorProcessorId {
    pub(crate) executor_processor_id: i64,
}

#[derive(Queryable, AsChangeset, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "executor_processor"]

pub struct ExecutorSelection {
    id: i64,
    #[serde(rename(serialize = "title"))]
    name: String,
}

pub(crate) struct ExecutorProcessorQueryBuilder;
impl ExecutorProcessorQueryBuilder {
    pub(crate) fn query_all_columns() -> executor_processor::BoxedQuery<'static, Mysql> {
        executor_processor::table
            .into_boxed()
            .select(executor_processor::all_columns)
    }

    pub(crate) fn query_selection_columns(
    ) -> executor_processor::BoxedQuery<'static, Mysql, (sql_types::Bigint, sql_types::VarChar)>
    {
        executor_processor::table
            .into_boxed()
            .select((executor_processor::id, executor_processor::name))
    }
    pub(crate) fn query_count(
    ) -> executor_processor::BoxedQuery<'static, Mysql, diesel::sql_types::Bigint> {
        executor_processor::table.into_boxed().count()
    }
}

impl QueryParamsExecutorProcessor {
    pub(crate) fn query_filter<ST>(
        self,
        mut statement_builder: executor_processor::BoxedQuery<'static, Mysql, ST>,
    ) -> executor_processor::BoxedQuery<'static, Mysql, ST> {
        if let Some(executor_processor_id) = self.id {
            statement_builder =
                statement_builder.filter(executor_processor::id.eq(executor_processor_id));
        }

        if let Some(machine_id) = self.machine_id {
            statement_builder =
                statement_builder.filter(executor_processor::machine_id.eq(machine_id));
        }

        if let Some(executor_processor_name) = self.name {
            statement_builder =
                statement_builder.filter(executor_processor::name.like(executor_processor_name));
        }

        if let Some(executor_processor_description) = self.description {
            statement_builder = statement_builder
                .filter(executor_processor::description.like(executor_processor_description));
        }

        if let Some(executor_processor_tag) = self.tag {
            statement_builder =
                statement_builder.filter(executor_processor::tag.like(executor_processor_tag));
        }

        statement_builder.order(executor_processor::id.desc())
    }
}

#[cached(
    type = "TimedSizedCache<i64, Option<String>>",
    create = "{ TimedSizedCache::with_size_and_lifespan(1024, 60) }",
    convert = r#"{ id }"#
)]
pub(crate) async fn get_executor_token_by_id(id: i64, conn: db::PoolConnection) -> Option<String> {
    use db::schema::executor_processor;

    web::block(move || {
        executor_processor::table
            .find(id)
            .select(executor_processor::token)
            .first::<String>(&conn)
    })
    .await
    .ok()
}
