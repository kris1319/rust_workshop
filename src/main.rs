use cxx::let_cxx_string;

#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(Clone, Copy, Debug)]
    enum IndexType {
        Tree,
        HashMap,
    }

    #[derive(Clone, Debug)]
    pub struct IndexResult {
        name: String,
        positions: Vec<u32>,
    }

    unsafe extern "C++" {
        include!("rust_cxx/include/index.h");

        type IndexType;
    }

    unsafe extern "C++" {
        include!("rust_cxx/include/indexshim.h");
        
        type Index;

        fn new_index(type_: IndexType) -> UniquePtr<Index>;

        fn dictionary_size(&self) -> i32;
        fn index_type(&self) -> IndexType;

        fn index(self: Pin<&mut Index>, name: &CxxString, text: &CxxString);

        fn search_index(index: &Index, word: &str) -> Vec<String>;

        fn search_index_with_positions(index: &Index, word: &str) -> Vec<IndexResult>;
    }
}

fn main() {
    let mut index = ffi::new_index(ffi::IndexType::HashMap);
    println!("Hello, world!");
    println!("Index size: {:?}", index.dictionary_size());
    println!("Index type: {:?}", index.index_type());

    let_cxx_string!(name = "doc1");
    let_cxx_string!(text = "This is a test document with 10 words to index.");
    index.pin_mut().index(&name, &text);
    println!("Index size after indexing: {:?}", index.dictionary_size());

    let_cxx_string!(name = "doc2");
    let_cxx_string!(text = "This is a second test document with some random ending.");
    index.pin_mut().index(&name, &text);
    println!("Index size after 2 indexing: {:?}", index.dictionary_size());

    let docs = ffi::search_index(&index, "test");
    println!("Search results for 'test': {:?}", docs);

    let results = ffi::search_index_with_positions(&index, "test");
    for result in results {
        println!("Document: {}, Positions: {:?}", result.name, result.positions);
    }
}
