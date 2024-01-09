use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use tera::Tera;

fn main() {
    // Set up Tera template engine
    let tera = Tera::new("templates/**/*.html").expect("Failed to initialize Tera");

    let content_dir = "content";
    let output_dir = "output";
    let static_dir = "static";
    let css_file_path = format!("{}/yocto.css", static_dir);

    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir).unwrap();

    // Read the YoctoCSS content
    let yocto_css_content = fs::read_to_string(&css_file_path).unwrap_or_else(|_| {
        eprintln!("Failed to read YoctoCSS file. Make sure it exists at: {}", css_file_path);
        String::new()
    });

    // Read each Markdown file in the content directory
    let entries = fs::read_dir(content_dir).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let file_path = entry.path();
        if file_path.is_file() && file_path.extension().unwrap_or_default() == "md" {
            let file_name = file_path.file_stem().unwrap().to_str().unwrap();
            let output_path = format!("{}/{}.html", output_dir, file_name);

            // Read content from the Markdown file
            let markdown_content = fs::read_to_string(&file_path).unwrap();

            // Apply template and generate HTML
            let mut context = tera::Context::new();
            context.insert("title", file_name);
            context.insert("content", &markdown_to_html(&markdown_content));
            context.insert("yocto_css", &yocto_css_content);

            let html_content = tera.render("article.html", &context).expect("Failed to render template");

            // Write HTML to the output file
            fs::write(&output_path, &html_content).unwrap();
        }
    }
}

fn markdown_to_html(markdown: &str) -> String {
    // You can use your preferred Markdown to HTML conversion library here
    // For simplicity, let's use the pulldown-cmark crate for basic conversion
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_TABLES);
    let parser = pulldown_cmark::Parser::new_ext(markdown, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    html_output
}
