// use juniper::{EmptyMutation, RootNode};
// use rocket::state;

pub mod cats;

// type Schema = RootNode<'static, Database, EmptyMutation<Database>>;
//
// #[get("/graphql")]
// fn graphql() -> content::Html<String> {
//     juniper_rocket::graphiql_source("/graphql")
// }
//
// #[get("/graphql?<request>")]
// fn get_graphql_handler(
//     context: State<Database>, // import DB
//     request: juniper_rocket::GraphQLRequest,
//     schema: State<Schema>
// ) -> juniper_rocket::GraphQLRequest {
//     request.execute(&schema, &context)
// }
