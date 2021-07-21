// DO NOT EDIT THIS FILE
// This file was generated by https://github.com/tacogips/async-graphql-reverse
use super::enums::SortDirection;
use super::enums::Status;
use super::input_objects::CreateFriendMutationInput;
use super::scalars::Url;
use super::unions::SearchResult;
use crate::datasource::DataSource;
use async_graphql::*;
#[derive(Debug, Clone)]
pub struct Query {}
#[Object]
impl Query {
    #[doc = "me: Single-line comment"]
    pub async fn me(&self, ctx: &Context<'_>) -> Result<Me> {
        ctx.data_unchecked::<DataSource>()
            .query_me(&ctx, self)
            .await
    }
    pub async fn active(&self, ctx: &Context<'_>) -> Result<bool> {
        ctx.data_unchecked::<DataSource>()
            .query_active(&ctx, self)
            .await
    }
    pub async fn r#type(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        ctx.data_unchecked::<DataSource>()
            .query_type(&ctx, self)
            .await
    }
    pub async fn custom_resolver(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        ctx.data_unchecked::<DataSource>()
            .query_custom_resolver(&ctx, self)
            .await
    }
}
#[derive(Debug, Clone)]
pub struct Mutation {}
#[Object]
impl Mutation {
    pub async fn create_friend_mutation(
        &self,
        ctx: &Context<'_>,
        input: CreateFriendMutationInput,
    ) -> Result<Option<CreateFriendMutationPayload>> {
        ctx.data_unchecked::<DataSource>()
            .mutation_create_friend_mutation(&ctx, self, input)
            .await
    }
}
#[derive(Debug, Clone)]
pub struct Subscription {
    pub badge: i64,
}
#[Object]
impl Subscription {
    pub async fn badge(&self) -> i64 {
        self.badge
    }
}
#[derive(Debug, Clone)]
pub struct CreateFriendMutationPayload {}
#[Object]
impl CreateFriendMutationPayload {
    pub async fn friend(&self, ctx: &Context<'_>) -> Result<Friend> {
        ctx.data_unchecked::<DataSource>()
            .create_friend_mutation_payload_friend(&ctx, self)
            .await
    }
}
#[derive(Debug, Clone)]
pub struct Friend {
    pub id: ID,
    this_is_a_hidden_field: String,
    this_is_another_hidden_field: i64,
}
#[Object]
impl Friend {
    pub async fn id(&self) -> ID {
        self.id.clone()
    }
    pub async fn name(&self, ctx: &Context<'_>) -> Result<String> {
        ctx.data_unchecked::<DataSource>()
            .friend_name(&ctx, self)
            .await
    }
    pub async fn others(&self, ctx: &Context<'_>) -> Result<Vec<Option<Friend>>> {
        ctx.data_unchecked::<DataSource>()
            .friend_others(&ctx, self)
            .await
    }
}
#[derive(Debug, Clone)]
pub struct FriendConnection {
    pub total_count: i64,
}
#[Object]
impl FriendConnection {
    pub async fn total_count(&self) -> i64 {
        self.total_count
    }
    pub async fn nodes(&self, ctx: &Context<'_>) -> Result<Vec<Option<Friend>>> {
        ctx.data_unchecked::<DataSource>()
            .friend_connection_nodes(&ctx, self)
            .await
    }
}
#[derive(Debug, Clone)]
pub struct Me {
    pub id: ID,
    pub name: String,
    pub rank: f64,
    pub email: Option<String>,
    pub age: Option<i64>,
    pub active: Option<bool>,
    pub web: Option<Url>,
    pub search_second: Vec<SearchResult>,
}
#[Object]
impl Me {
    pub async fn id(&self) -> ID {
        self.id.clone()
    }
    pub async fn name(&self) -> String {
        self.name.clone()
    }
    pub async fn rank(&self) -> f64 {
        self.rank
    }
    pub async fn email(&self) -> Option<String> {
        self.email.clone()
    }
    pub async fn age(&self) -> Option<i64> {
        self.age
    }
    pub async fn active(&self) -> Option<bool> {
        self.active
    }
    pub async fn friends(
        &self,
        ctx: &Context<'_>,
        first: Option<i64>,
        limit: Option<i64>,
        sort_direction: Option<SortDirection>,
        next_token: Option<String>,
    ) -> Result<FriendConnection> {
        ctx.data_unchecked::<DataSource>()
            .me_friends(&ctx, self, first, limit, sort_direction, next_token)
            .await
    }
    pub async fn notifications(&self, ctx: &Context<'_>) -> Result<Vec<Option<Notification>>> {
        ctx.data_unchecked::<DataSource>()
            .me_notifications(&ctx, self)
            .await
    }
    pub async fn web(&self) -> Option<Url> {
        self.web.clone()
    }
    pub async fn search(
        &self,
        ctx: &Context<'_>,
        text: String,
    ) -> Result<Vec<Option<SearchResult>>> {
        ctx.data_unchecked::<DataSource>()
            .me_search(&ctx, self, text)
            .await
    }
    pub async fn search_second(&self) -> Vec<SearchResult> {
        self.search_second.clone()
    }
    pub async fn status(&self, ctx: &Context<'_>) -> Result<Option<Status>> {
        ctx.data_unchecked::<DataSource>()
            .me_status(&ctx, self)
            .await
    }
}
#[derive(Debug, Clone)]
pub struct Notification {
    pub id: ID,
    pub title: String,
}
#[Object]
impl Notification {
    pub async fn id(&self) -> ID {
        self.id.clone()
    }
    pub async fn title(&self) -> String {
        self.title.clone()
    }
    pub async fn friends(
        &self,
        ctx: &Context<'_>,
        first: Option<i64>,
        num: Option<i64>,
    ) -> Result<FriendConnection> {
        ctx.data_unchecked::<DataSource>()
            .notification_friends(&ctx, self, first, num)
            .await
    }
}
