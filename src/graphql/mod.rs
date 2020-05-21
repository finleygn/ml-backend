use juniper::{FieldResult, Variables, EmptyMutation, RootNode, GraphQLObject};

#[derive(GraphQLObject)]
struct Artist {
    id: i32,
    name: String,
}

#[derive(GraphQLObject)]
struct Song {
    id: i32,
    name: String,
    artist: Artist,
}
//
// #[juniper::graphql_object(
//     description = "A person!",
// )]
// impl Artist {
//     fn id(&self) -> u32 {
//         self.id
//     }
//
//     fn name(&self) -> &str {
//         self.name.as_str()
//     }
// }
//
// #[juniper::graphql_object(
//     description = "A song!",
// )]
// impl Song {
//     fn id(&self) -> u32 {
//         self.id
//     }
//
//     fn name(&self) -> &str {
//         self.name.as_str()
//     }
//
//     fn artist(&self) -> &Artist {
//         &self.artist
//     }
// }


/**
 * Query
 */
pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn song(id: i32) -> FieldResult<Vec<Song>> {
        let artist = Artist {
            id: 1,
            name: String::from("test"),
        };

        Ok(vec![
            Song {
                id,
                name: String::from("test2"),
                artist,
            }
        ])
    }

    fn artist(id: i32) -> FieldResult<Vec<Artist>> {
        let artist = Artist {
            id,
            name: String::from("test"),
        };

        Ok(vec![
            artist
        ])
    }
}


/**
 * Schema
 */
pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}