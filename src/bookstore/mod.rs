mod model;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use model::Organization;
use slab::Slab;
use std::collections::HashMap;

pub use model::QueryRoot;
pub type BookStoreSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct BookStoreUser{
    id: &'static str,
    name: &'static str,
    //postings: Slab<Posting>,
    institution:Vec<Organization>
}

pub struct Book{
    id: &'static str,
    title: &'static str,
}

pub struct Posting{
    id: &'static str,
    user: usize,
    book: usize
}

pub struct BookStore{
    users:Slab<BookStoreUser>,
    students:HashMap<&'static str,usize>,
    faculties:HashMap<&'static str,usize>,

    book:Slab<Book>,
    books:HashMap<&'static str,usize>,

    posting:Slab<Posting>,
    postings:HashMap<&'static str,usize>
}
impl BookStore{
    #[allow(clippy::new_without_default)]
    pub fn new()->Self{
        let mut users = Slab::new();
        let student1=users.insert(BookStoreUser{
            id:"1001",
            name:"Katharine Hepburn",
            //postings:vec![],
            institution:vec![Organization::Conestoga]
        });
        let student2=users.insert(BookStoreUser{
            id:"1002",
            name:"Denzel Washington",
            //postings:vec![],
            institution:vec![Organization::Laurier]
        });
        let student3=users.insert(BookStoreUser{
            id:"1003",
            name:"Marlon Brando",
            //postings:vec![],
            institution:vec![Organization::Waterloo]
        });
        let faculty1=users.insert(BookStoreUser{
            id:"2001",
            name:"Jack Nicholson",
            //postings:vec![],
            institution:vec![Organization::Waterloo]
        });
        let faculty2=users.insert(BookStoreUser{
            id:"2002",
            name:"Robert De Niro",
            //postings:vec![],
            institution:vec![Organization::Waterloo,Organization::Conestoga]
        });

        let mut book = Slab::new();
        let book1=book.insert(Book{
            id:"3001",
            title:"Algebra1"
        });
        let book2=book.insert(Book{
            id:"3002",
            title:"Algebra2"
        });
        let book3=book.insert(Book{
            id:"3003",
            title:"Code complete2"
        });

        let mut post=Slab::new();
        let posting1=post.insert(Posting{
            id:"4001",
            user:student1,
            book:book3
        });
        let posting2=post.insert(Posting{
            id:"4002",
            user:student2,
            book:book2,
        });
        let posting3=post.insert(Posting{
            id:"4003",
            user:faculty1,
            book:book1
        });


        let mut students = HashMap::new();
        students.insert("1001",student1);
        students.insert("1002",student2);
        students.insert("1003",student3);


        let mut faculties = HashMap::new();
        faculties.insert("2001",faculty1);
        faculties.insert("2002",faculty2);

        let mut books = HashMap::new();
        books.insert("3001",book1);
        books.insert("3002",book2);
        books.insert("3003",book3);

        let mut postings=HashMap::new();
        postings.insert("4001",posting1);
        postings.insert("4002",posting2);
        postings.insert("4003",posting3);

        // users[student1].postings=vec![posting1];
        // users[student2].postings=vec![posting2];
        // users[faculty1].postings=vec![posting3];

        Self{
            users,
            students,
            faculties,
            book,
            books,
            posting:post,
            postings
        }

    }
    pub fn student(&self,id: &str)->Option<usize>{
        self.students.get(id).cloned()
    }
    pub fn students(&self)->Vec<usize>{
        self.students.values().cloned().collect()
    }
    pub fn faculty(&self,id: &str)->Option<usize>{
        self.faculties.get(id).cloned()
    }
    pub fn faculties(&self)->Vec<usize>{
        self.faculties.values().cloned().collect()
    }

    pub fn book(&self,id: &str)->Option<usize>{
        self.books.get(id).cloned()
    }
    pub fn books(&self)->Vec<usize>{
        self.books.values().cloned().collect()
    }

    pub fn posting(&self,id: &str)->Option<usize>{
        self.postings.get(id).cloned()
    }
    pub fn postings(&self)->Vec<usize>{
        self.postings.values().cloned().collect()
    }
}

