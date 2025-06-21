# Personal Portfolio

A modern, responsive personal portfolio website built with Rust and Actix Web framework. This portfolio showcases my projects, skills, and professional experience through a fast and efficient web application.

## Tech Stack

- **Backend**: Rust with Actix Web
- **Templating**: Askama (Rust template engine)
- **HTTP Client**: Reqwest for external API calls
- **Configuration**: TOML and dotenv for environment management
- **Serialization**: Serde for JSON handling

## Dependencies

```toml
actix-files = "0.6.6"     
actix-web = "4.11.0"      
askama = "0.14.0"          
dotenv = "0.15.0"         
reqwest = "0.12.20"     
serde = "1.0.219"
toml = "0.8.23"            
```

## 📁 Project Structure

```
portfolio/
├── src/
│   ├──  main.rs          # Application entry point
│ 
│  
├── templates/           # Askama HTML templates
├── static/             # CSS, JS, images, and other assets
├── data/                # Raw Data files
├── .env                # Environment variables
├── Cargo.toml          # Project dependencies
└── README.md
```

### Manual Deployment

1. Build the release version
2. Copy the binary and static files to your server
3. Set up environment variables
4. Run the application with a process manager like systemd


## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Actix Web](https://actix.rs/) - Fast, powerful web framework for Rust
- [Askama](https://docs.rs/askama/) - Type-safe, compiled Jinja-like templates
- [Rust Community](https://www.rust-lang.org/community) - For the amazing ecosystem

---

⭐ Don't forget to give the project a star if you like
