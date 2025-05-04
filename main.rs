use sysinfo::{System, SystemExt};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use std::error::Error;

fn get_ram_info() -> String {
    let mut system = System::new_all();
    system.refresh_all();
    
    let total_memory = system.total_memory();
    let used_memory = system.used_memory();
    let free_memory = system.free_memory();
    
    format!(
        "RAM Information:\nTotal: {} MB\nUsed: {} MB\nFree: {} MB",
        total_memory / 1024,
        used_memory / 1024,
        free_memory / 1024
    )
}

fn send_email(ram_info: &str) -> Result<(), Box<dyn Error>> {
    // Replace with your Gmail credentials
    let email = "your_email@gmail.com";
    let password = "your_app_password"; // Use Gmail App Password, not regular password
    
    let email = Message::builder()
        .from(email.parse()?)
        .to(email.parse()?) // Sending to self, modify as needed
        .subject("System RAM Report")
        .body(ram_info.to_string())?;
    
    let creds = Credentials::new(email.to_string(), password.to_string());
    
    // Gmail SMTP server
    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();
    
    // Send the email
    mailer.send(&email)?;
    
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let ram_info = get_ram_info();
    println!("Collecting RAM info:\n{}", ram_info);
    
    send_email(&ram_info)?;
    println!("Email sent successfully!");
    
    Ok(())
}