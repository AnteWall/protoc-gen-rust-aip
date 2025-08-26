/*!
 * Test application for the protoc-gen-rust-aip generated library.
 * 
 * This application demonstrates the usage of the refactored library that uses
 * tonic/prost for protobuf generation and includes AIP resource name support.
 * 
 * The test covers:
 * - Basic generated protobuf types (Book, Shelf, Author, Review, Publisher)
 * - AIP resource name types and validation
 * - gRPC client type creation and usage
 */

use build_with_buf::example::library::v1::*;
use build_with_buf::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing the refactored protoc-gen-rust-aip library...");
    
    // Test creating and using generated types
    test_basic_types().await?;
    
    // Test AIP resource names
    test_resource_names().await?;
    
    // Test gRPC client functionality
    test_grpc_client().await?;
    
    println!("All tests passed successfully!");
    Ok(())
}

async fn test_basic_types() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Testing Basic Generated Types ---");
    
    // Test Book creation
    let book = Book {
        name: "projects/my-project/books/rust-programming".to_string(),
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik".to_string(),
    };
    
    println!("Created Book: {} by {}", book.title, book.author);
    
    // Test Shelf creation
    let shelf = Shelf {
        name: "projects/my-project/shelves/programming".to_string(),
        display_name: "Programming Books".to_string(),
    };
    
    println!("Created Shelf: {}", shelf.display_name);
    
    // Test Author creation
    let author = Author {
        name: "authors/steve-klabnik".to_string(),
        display_name: "Steve Klabnik".to_string(),
        biography: "Rust core team member and author".to_string(),
    };
    
    println!("Created Author: {}", author.display_name);
    
    // Test Review creation
    let review = Review {
        name: "projects/my-project/books/rust-programming/reviews/review-1".to_string(),
        content: "Excellent introduction to Rust!".to_string(),
        rating: 5,
    };
    
    println!("Created Review with rating: {}/5", review.rating);
    
    // Test Publisher creation
    let publisher = Publisher {
        name: "publishers/no-starch".to_string(),
        display_name: "No Starch Press".to_string(),
        website: "https://nostarch.com".to_string(),
    };
    
    println!("Created Publisher: {}", publisher.display_name);
    
    // Test new bookstore types from the second proto file
    use build_with_buf::example::bookstore::v1::*;
    
    let store = Store {
        name: "stores/downtown-books".to_string(),
        display_name: "Downtown Books".to_string(),
        address: "123 Main St, City".to_string(),
    };
    
    println!("Created Store: {} at {}", store.display_name, store.address);
    
    let category = Category {
        name: "stores/downtown-books/categories/fiction".to_string(),
        display_name: "Fiction".to_string(),
        description: "Fictional literature and novels".to_string(),
    };
    
    println!("Created Category: {}", category.display_name);
    
    Ok(())
}

async fn test_resource_names() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Testing AIP Resource Names ---");
    
    // Test BookResourceName
    let book_name = BookResourceName::new("my-project", "rust-programming");
    println!("Book resource name: {}", book_name.to_string());
    println!("Book resource type: {}", book_name.resource_type());
    
    // Test validation
    book_name.validate().expect("Book name should be valid");
    println!("Book name validation: PASSED");
    
    // Test invalid resource name
    let invalid_book = BookResourceName::new("", "book");
    match invalid_book.validate() {
        Err(e) => println!("Invalid book validation correctly failed: {}", e),
        Ok(_) => return Err("Expected validation to fail for empty project".into()),
    }
    
    // Test ProjectsShelfResourceName
    let projects_shelf = ProjectsShelfResourceName::new("my-project", "programming");
    println!("Projects shelf resource name: {}", projects_shelf.to_string());
    projects_shelf.validate().expect("Projects shelf name should be valid");
    println!("Projects shelf name validation: PASSED");
    
    // Test UsersShelfResourceName  
    let users_shelf = UsersShelfResourceName::new("john-doe", "favorites");
    println!("Users shelf resource name: {}", users_shelf.to_string());
    users_shelf.validate().expect("Users shelf name should be valid");
    println!("Users shelf name validation: PASSED");
    
    // Test ShelfResourceName enum
    let shelf_from_projects = ShelfResourceName::Projects(projects_shelf);
    println!("Shelf enum (projects): {}", shelf_from_projects);
    shelf_from_projects.validate().expect("Shelf enum should be valid");
    
    let shelf_from_users = ShelfResourceName::Users(users_shelf);
    println!("Shelf enum (users): {}", shelf_from_users);
    shelf_from_users.validate().expect("Shelf enum should be valid");
    
    // Test parsing shelf resource names
    let parsed_shelf = parse_shelf_resource_name("projects/my-project/shelves/programming")?;
    println!("Parsed shelf resource name: {}", parsed_shelf);
    
    // Test new bookstore resource names
    let store_name = StoreResourceName::new("downtown-books");
    println!("Store resource name: {}", store_name.to_string());
    println!("Store resource type: {}", store_name.resource_type());
    store_name.validate().expect("Store name should be valid");
    println!("Store name validation: PASSED");
    
    let category_name = CategoryResourceName::new("downtown-books", "fiction");
    println!("Category resource name: {}", category_name.to_string());
    println!("Category resource type: {}", category_name.resource_type());
    category_name.validate().expect("Category name should be valid");
    println!("Category name validation: PASSED");
    
    Ok(())
}

async fn test_grpc_client() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Testing gRPC Client Setup ---");
    
    // Test request creation
    let list_request = ListBooksRequest {
        parent: "projects/my-project".to_string(),
        page_size: 10,
        page_token: "".to_string(),
    };
    
    println!("Created ListBooksRequest for parent: {}", list_request.parent);
    
    let get_request = GetBookRequest {
        name: "projects/my-project/books/rust-programming".to_string(),
    };
    
    println!("Created GetBookRequest for book: {}", get_request.name);
    
    let resource_request = GetResourceRequest {
        name: "projects/my-project/books/rust-programming".to_string(),
    };
    
    println!("Created GetResourceRequest for resource: {}", resource_request.name);
    
    // Note: We're not actually connecting to a gRPC server here,
    // just testing that the client types can be created and used
    println!("gRPC client types created successfully");
    
    Ok(())
}
