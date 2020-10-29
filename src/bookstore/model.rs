use super::BookStore;

use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use async_graphql::{Context, Enum, FieldResult, Interface, Object};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Organization{
    Conestoga,
    Laurier,
    Waterloo,
}

pub struct Post(usize);
#[Object]
impl Post{
    async fn id(&self, ctx:&Context<'_>)->&str{
        ctx.data_unchecked::<BookStore>().posting[self.0].id
    }
    async fn user(&self, ctx:&Context<'_>)->BookStoreUser{
        //왠지 모르겠지만 Student(uszie) 결과값이 0이여야만 될거 같음...
        if Student(ctx.data_unchecked::<BookStore>().posting[self.0].user).0==0{
            println!("Get student. usize is {}",Student(ctx.data_unchecked::<BookStore>().posting[self.0].user).0);
            Student(ctx.data_unchecked::<BookStore>().posting[self.0].user).into()
        }else{
            println!("Gt faculty. usize is {}",Student(ctx.data_unchecked::<BookStore>().posting[self.0].user).0);
            Faculty(ctx.data_unchecked::<BookStore>().posting[self.0].user).into()
        }
    }
    async fn book(&self, ctx:&Context<'_>)->Book{
        Book(ctx.data_unchecked::<BookStore>().posting[self.0].book)
    }

}


pub struct Student(usize);

#[Object]
impl Student{
    async fn id(&self, ctx:&Context<'_>)-> &str{
        ctx.data_unchecked::<BookStore>().users[self.0].id
    }
    async fn name(&self, ctx:&Context<'_>)-> &str{
        ctx.data_unchecked::<BookStore>().users[self.0].name
    }
    async fn institution<'a>(&self, ctx:&'a Context<'_>)-> &'a [Organization]{
        &ctx.data_unchecked::<BookStore>().users[self.0].institution
    }
}


pub struct Faculty(usize);
//BookSotre의 users는 BookStoreUser타입을 가진 slab 자료구조이다. 
#[Object]
impl Faculty{
    async fn id(&self, ctx:&Context<'_>)-> &str{
        ctx.data_unchecked::<BookStore>().users[self.0].id
    }
    async fn name(&self, ctx:&Context<'_>)-> &str{
        ctx.data_unchecked::<BookStore>().users[self.0].name
    }
    async fn institution<'a>(&self, ctx:&'a Context<'_>)-> &'a [Organization]{
        &ctx.data_unchecked::<BookStore>().users[self.0].institution
    }
}

pub struct Book(usize);
#[Object]
impl Book{
    //여기서 self는 Book구조체를 가리키고 필드의 이름 없이 usize타입을 가진 튜플 구조체, id랑 title은 함수이다
    async fn id(&self,ctx:&Context<'_>)-> &str{
        ctx.data_unchecked::<BookStore>().book[self.0].id
    }
    async fn title(&self, ctx:&Context<'_>)->&str{
        ctx.data_unchecked::<BookStore>().book[self.0].title
    }
}

pub struct QueryRoot;
#[Object]
impl QueryRoot{
    // async fn user(
    //     &self,
    //     ctx: &Context<'_>,
    //     #[graphql(desc = "If omitted, returns the hero of the whole saga. If provided, returns the hero of that particular episode.")]
    //     institution:Organization
    // ) -> BookStoreUser {
    //     if institution == Organization::Conestoga{

    //     }else if institution == Organization::Laurier{

    //     }else {

    //     }
    // }
    async fn student(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the student")] id: String,
    )->Option<Student>{
        // 여기서 map에 구조체를 넣는 이유를 모르겠음...
        ctx.data_unchecked::<BookStore>().student(&id).map(Student)
    }
    async fn students(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    )->FieldResult<Connection<usize, Student, EmptyFields, EmptyFields>>{
        let students=ctx.data_unchecked::<BookStore>()
                    .students()
                    .iter()
                    .copied()
                    .collect::<Vec<_>>();
                    
        // 마지막 인자의 타입은 &[usize]
        query_users(after, before, first, last, &students).await.map(|conn| conn.map_node(Student))
    }
    async fn faculty(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the faculty")] id: String,
    )->Option<Faculty>{
        ctx.data_unchecked::<BookStore>().faculty(&id).map(Faculty)
    }
    async fn faculties(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    )->FieldResult<Connection<usize, Faculty, EmptyFields, EmptyFields>>{
        let faculties=ctx.data_unchecked::<BookStore>()
                    .faculties()
                    .iter()
                    .copied()
                    .collect::<Vec<_>>();
                    
        // 마지막 인자의 타입은 &[usize]
        query_users(after, before, first, last, &faculties).await.map(|conn| conn.map_node(Faculty))
    }

    async fn book(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the book")] id: String,
    )->Option<Book>{
        ctx.data_unchecked::<BookStore>().book(&id).map(Book)
    }

    async fn books(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    )->FieldResult<Connection<usize, Book, EmptyFields, EmptyFields>>{
        let books=ctx.data_unchecked::<BookStore>()
                    .books()
                    .iter()
                    .copied()
                    .collect::<Vec<_>>();
                    
        query_users(after, before, first, last, &books).await.map(|conn| conn.map_node(Book))
    }

    async fn posting(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the posting")] id: String,
    )->Option<Post>{
        ctx.data_unchecked::<BookStore>().posting(&id).map(Post)
    }

    async fn postings(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    )->FieldResult<Connection<usize, Post, EmptyFields, EmptyFields>>{
        let postings=ctx.data_unchecked::<BookStore>()
                    .postings()
                    .iter()
                    .copied()
                    .collect::<Vec<_>>();
                    
        query_users(after, before, first, last, &postings).await.map(|conn| conn.map_node(Post))
    }
}

//여기서 정하는것이 나중에 graphiql의 doc에서 반영된다
#[derive(Interface)]
#[graphql(
    field(name = "id", type = "&str"),
    field(name = "name", type = "&str"),
    field(name = "institution", type = "&'ctx [Organization]")
)]
pub enum BookStoreUser{
    Student(Student),
    Faculty(Faculty),
}


async fn query_users(
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    users: &[usize],
) -> FieldResult<Connection<usize, usize, EmptyFields, EmptyFields>> {
    query(
        after,
        before,
        first,
        last,
        |after, before, first, last| async move {
            let mut start = 0usize;
            let mut end = users.len();

            if let Some(after) = after {
                if after >= users.len() {
                    return Ok(Connection::new(false, false));
                }
                start = after + 1;
            }

            if let Some(before) = before {
                if before == 0 {
                    return Ok(Connection::new(false, false));
                }
                end = before;
            }

            let mut slice = &users[start..end];

            if let Some(first) = first {
                slice = &slice[..first.min(slice.len())];
                end -= first.min(slice.len());
            } else if let Some(last) = last {
                slice = &slice[slice.len() - last.min(slice.len())..];
                start = end - last.min(slice.len());
            }

            let mut connection = Connection::new(start > 0, end < users.len());
            connection.append(
                slice
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| Edge::new(start + idx, *item)),
            );
            Ok(connection)
        },
    )
    .await
}
