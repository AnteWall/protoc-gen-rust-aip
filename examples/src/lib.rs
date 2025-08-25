//! Comprehensive example demonstrating generated AIP resource names.
//!
//! This library showcases all types of resource name patterns:
//! - Single pattern resources (Book)
//! - Multi-pattern resources (Shelf)
//! - Nested resources (Review)
//! - Future multi-pattern resources (Author)
//! - Complex multi-pattern resources (Publisher)

pub mod gen {
    //! Generated AIP resource name types.
    include!("github.com/AnteWall/protoc-gen-rust-aip/examples/comprehensive/gen/library_aip.rs");
}

use gen::*;
use std::str::FromStr;

/// Example usage of single-pattern resource names.
pub mod single_pattern {
    use super::*;

    /// Demonstrates BookResourceName usage.
    pub fn example_book_operations() -> Result<(), String> {
        // Create a new book resource name
        let book = BookResourceName::new("my-project", "rust-guide");
        println!("Created book: {book}");

        // Validate the resource name
        book.validate()?;
        println!("Book validation passed");

        // Parse from string
        let book_str = "projects/example-project/books/learning-rust";
        let parsed_book = BookResourceName::from_str(book_str)?;
        println!("Parsed book: {parsed_book}");

        // Check for wildcards
        let wildcard_book = BookResourceName::new("my-project", "-");
        let contains_wildcard = wildcard_book.contains_wildcard();
        println!("Book contains wildcard: {contains_wildcard}");

        // Get resource type
        let resource_type = book.resource_type();
        println!("Book resource type: {resource_type}");

        Ok(())
    }
}

/// Example usage of multi-pattern resource names.
pub mod multi_pattern {
    use super::*;

    /// Demonstrates ShelfResourceName multi-pattern usage.
    pub fn example_shelf_operations() -> Result<(), String> {
        // Create project-scoped shelf
        let project_shelf = ProjectsShelfResourceName::new("my-project", "fiction");
        let shelf_enum = ShelfResourceName::Projects(project_shelf);
        println!("Created project shelf: {shelf_enum}");

        // Create user-scoped shelf
        let user_shelf = UsersShelfResourceName::new("alice", "favorites");
        let user_shelf_enum = ShelfResourceName::Users(user_shelf);
        println!("Created user shelf: {user_shelf_enum}");

        // Parse from string - the enum handles the pattern matching
        let shelf_str1 = "projects/library-project/shelves/science";
        let parsed_shelf1 = parse_shelf_resource_name(shelf_str1)?;
        println!("Parsed shelf (project): {parsed_shelf1}");

        let shelf_str2 = "users/bob/shelves/technical";
        let parsed_shelf2 = parse_shelf_resource_name(shelf_str2)?;
        println!("Parsed shelf (user): {parsed_shelf2}");

        // Use enum methods
        let resource_type = parsed_shelf1.resource_type();
        println!("Shelf resource type: {resource_type}");
        let contains_wildcard = parsed_shelf1.contains_wildcard();
        println!("Shelf contains wildcard: {contains_wildcard}");

        Ok(())
    }
}

/// Example usage of nested resource names.
pub mod nested_resources {
    use super::*;

    /// Demonstrates ReviewResourceName nested resource usage.
    pub fn example_review_operations() -> Result<(), String> {
        // Create a review for a book
        let review = ReviewResourceName::new("library-project", "rust-book", "review-123");
        println!("Created review: {review}");

        // Validate the nested resource
        review.validate()?;
        println!("Review validation passed");

        // Parse from string
        let review_str = "projects/my-project/books/learning-go/reviews/helpful-review";
        let parsed_review = ReviewResourceName::from_str(review_str)?;
        println!("Parsed review: {parsed_review}");

        // Access parent resource information
        let book = &parsed_review.book;
        let project = &parsed_review.project;
        println!("Review's book: {book}");
        println!("Review's project: {project}");

        Ok(())
    }
}

/// Example usage of future multi-pattern resources.
pub mod future_multi_pattern {
    use super::*;

    /// Demonstrates AuthorResourceName future multi-pattern usage.
    pub fn example_author_operations() -> Result<(), String> {
        // Create author (currently only one pattern, but prepared for future expansion)
        let project_author = AuthorsAuthorResourceName::new("jane-doe");
        let author_enum = AuthorResourceName::Authors(project_author);
        println!("Created author: {author_enum}");

        // Parse from string
        let author_str = "authors/john-smith";
        let parsed_author = parse_author_resource_name(author_str)?;
        println!("Parsed author: {parsed_author}");

        // Future patterns can be added without breaking existing code
        let resource_type = parsed_author.resource_type();
        println!("Author resource type: {resource_type}");

        Ok(())
    }
}

/// Example usage of complex multi-pattern resources.
pub mod complex_multi_pattern {
    use super::*;

    /// Demonstrates PublisherResourceName complex multi-pattern usage.
    pub fn example_publisher_operations() -> Result<(), String> {
        // Create direct publisher (no prefix)
        let direct_publisher = PublishersPublisherResourceName::new("tech-books");
        let direct_enum = PublisherResourceName::Publishers(direct_publisher);
        println!("Created direct publisher: {direct_enum}");

        // Create organization-scoped publisher
        let org_publisher = OrganizationsPublisherResourceName::new("book-org", "academic-press");
        let org_enum = PublisherResourceName::Organizations(org_publisher);
        println!("Created organization publisher: {org_enum}");

        // Create project-scoped publisher
        let project_publisher =
            ProjectsPublisherResourceName::new("global-project", "local-publisher");
        let project_enum = PublisherResourceName::Projects(project_publisher);
        println!("Created project publisher: {project_enum}");

        // Parse various patterns
        let publisher_patterns = [
            "publishers/mystery-books",
            "organizations/global-org/publishers/science-press",
            "projects/publishing-co/publishers/history-books",
        ];

        for pattern in &publisher_patterns {
            let parsed = parse_publisher_resource_name(pattern)?;
            println!("Parsed publisher: {parsed}");
        }

        Ok(())
    }
}

/// Comprehensive example demonstrating all resource types.
pub fn run_all_examples() -> Result<(), String> {
    println!("=== Single Pattern Resources ===");
    single_pattern::example_book_operations()?;

    println!("\n=== Multi-Pattern Resources ===");
    multi_pattern::example_shelf_operations()?;

    println!("\n=== Nested Resources ===");
    nested_resources::example_review_operations()?;

    println!("\n=== Future Multi-Pattern Resources ===");
    future_multi_pattern::example_author_operations()?;

    println!("\n=== Complex Multi-Pattern Resources ===");
    complex_multi_pattern::example_publisher_operations()?;

    println!("\n✅ All examples completed successfully!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_name_round_trip() {
        let book = BookResourceName::new("test-project", "test-book");
        let book_str = book.to_string();
        let parsed = BookResourceName::from_str(&book_str).unwrap();
        assert_eq!(book, parsed);
    }

    #[test]
    fn test_shelf_multi_pattern() {
        let project_shelf = ProjectsShelfResourceName::new("proj", "shelf1");
        let user_shelf = UsersShelfResourceName::new("user1", "shelf2");

        // Both should parse correctly through the enum
        let parsed1 =
            parse_shelf_resource_name(&ShelfResourceName::Projects(project_shelf).to_string())
                .unwrap();
        let parsed2 =
            parse_shelf_resource_name(&ShelfResourceName::Users(user_shelf).to_string()).unwrap();

        assert_eq!(parsed1.resource_type(), "library.googleapis.com/Shelf");
        assert_eq!(parsed2.resource_type(), "library.googleapis.com/Shelf");
    }

    #[test]
    fn test_wildcard_detection() {
        let normal_book = BookResourceName::new("project", "book");
        let wildcard_book = BookResourceName::new("project", "-");

        assert!(!normal_book.contains_wildcard());
        assert!(wildcard_book.contains_wildcard());
    }

    #[test]
    fn test_validation() {
        let valid_book = BookResourceName::new("valid-project", "valid-book");
        let invalid_book = BookResourceName::new("", "book");

        assert!(valid_book.validate().is_ok());
        assert!(invalid_book.validate().is_err());
    }

    #[test]
    fn test_nested_resource() {
        let review = ReviewResourceName::new("proj", "book", "rev");
        let review_str = review.to_string();
        assert!(review_str.contains("projects/proj/books/book/reviews/rev"));

        let parsed = ReviewResourceName::from_str(&review_str).unwrap();
        assert_eq!(review, parsed);
    }
}
