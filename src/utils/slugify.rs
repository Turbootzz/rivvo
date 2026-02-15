use slug::slugify;

/// Creates a URL-friendly slug from the given input string.
/// Used for generating board and organization slugs.
#[allow(dead_code)]
pub fn create_slug(input: &str) -> String {
    slugify(input)
}
