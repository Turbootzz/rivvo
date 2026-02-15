use slug::slugify;

pub fn create_slug(input: &str) -> String {
    slugify(input)
}
