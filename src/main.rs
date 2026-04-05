// Standard library imports
use std::{
    env,
    fmt::{Display, Formatter},
    net::TcpListener,
};

// External crates
use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use calamine::{open_workbook_auto, Data, Reader};
use chrono::{Datelike, Duration, NaiveDate};
use serde::Serialize;
use sysinfo::System;
use tokio::time::sleep;

// Local modules
mod constants;
use constants::*;

const SERVER_ADDR: &str = "127.0.0.1:8080";

#[derive(Serialize)]
pub struct CalendarEvent {
    pub title: String,
    pub description: String,
    pub address: String,
}

impl Display for CalendarEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  Title: {}\n  Address: {}\n  Description:\n{}",
            if self.title.is_empty() {
                "No Title Found"
            } else {
                &self.title
            },
            if self.address.is_empty() {
                "No Address Found"
            } else {
                &self.address
            },
            if self.description.is_empty() {
                "  No Description Found"
            } else {
                &self.description
            }
        )
    }
}

#[get("/api/event-data")]
async fn get_event_data() -> impl Responder {
    println!("Attempting to load event data from Excel");
    let args: Vec<String> = env::args().collect();
    let event = if args.len() >= 3 {
        let file_path = &args[1];
        let row_num: usize = args[2].parse().unwrap_or(1);

        let mut workbook =
            open_workbook_auto(file_path).unwrap_or_else(|_| panic!("Cannot open file"));
        let sheet = workbook.worksheet_range_at(0).unwrap().unwrap();

        if let Some(row) = sheet.rows().nth(row_num - 1) {
            println!("  Found row {} in sheet", row_num);
            CalendarEvent {
                title: row[CALENDAR_ENTRY].to_string(),
                address: row[ADDRESS].to_string(),
                description: format_details(row),
            }
        } else {
            println!("  No data found in file");
            CalendarEvent {
                title: "Default Event".to_string(),
                address: "".to_string(),
                description: "No data found".to_string(),
            }
        }
    } else {
        println!("  No file provided");
        CalendarEvent {
            title: "".to_string(),
            address: "".to_string(),
            description: "".to_string(),
        }
    };
    println!("Data Scraped:\n{}", event);

    web::Json(event)
}

async fn exit() -> HttpResponse {
    // Schedule the exit after sending response
    actix_web::rt::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        std::process::exit(0);
    });

    HttpResponse::Ok().finish()
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../static/index.html"))
}

fn kill_other_instances() {
    let s = System::new_all();
    let current_pid = std::process::id();

    for (pid, process) in s.processes() {
        if process.name() == "job2event.exe" && pid.as_u32() != current_pid {
            println!("Closed Process ID: {}", pid.as_u32());
            process.kill();
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Clear previous instances of Job2Event
    kill_other_instances();
    if TcpListener::bind(SERVER_ADDR).is_err() {
        println!("Port 8080 is in use. Waiting for other instance to close...");
        sleep(std::time::Duration::from_secs(2)).await;
    } else {
        println!("Port 8080 is not in use. Creating new instance...");
    }

    // Create new instance of Job2Event
    let server = HttpServer::new(|| {
        App::new()
            .service(index)
            .service(get_event_data)
            .service(fs::Files::new("/static", "static").show_files_listing())
            .service(web::resource("/exit").route(web::post().to(exit)))
    })
    .bind(SERVER_ADDR)?;

    println!("Created new instance at http://localhost:8080/");
    webbrowser::open("http://localhost:8080/").expect("Failed to open URL");

    server.run().await
}

fn format_details(row: &[Data]) -> String {
    format!(
        "Job Name: {}\nDue Date: {}\nCust: {} {} {} {}\nTask: {}\nCoordination: {}\nParts: {}\nOnsite Contact: {} {}\nGC Info: {} {}\nPermit #: {}\nAddress: {}\nWater Purveyor: {}\nPO #: {}\nSame Day: {}\nScheduled: {}\nTravel: {}\nDate Contacted: {}\nNotes: {}\n",
        row[JOB_NAME],           // Job Name
        row[DUE_DATE],           // Due Date
        row[BILLING_FIRST_NAME], // Cust
        row[BILLING_LAST_NAME],  // Cust
        row[BILLING_COMPANY],    // Cust
        row[PHONE_NUMBER],       // Cust
        row[TASK],               // Task
        row[COORDINATION],       // Coordination
        row[PARTS],              // Parts
        row[ON_SITE_CONTACT],    // Onsite Contact
        row[ON_SITE_NUMBER],     // Onsite Contact
        row[GC_NAME],            // GC Info
        row[GC_NUMBER],          // GC Info
        row[PERMIT_NUMBER],      // Permit #
        row[ADDRESS],            // Address
        row[WATER_PURVEYOR],     // Water Purveyor
        row[PO_NUMBER],          // PO #
        row[SAME_DAY],           // Same Day
        row[SCHEDULED],          // Scheduled
        row[TRAVEL],             // Travel
        get_date_contacted(row),     // Date Contacted
        row[NOTES]               // Notes
    )
}

fn get_date_contacted(row: &[Data]) -> String {
    let serial = match &row[DATE_CONTACTED] {
        Data::String(s) => match s.parse::<f64>() {
            Ok(x) => x,
            Err(_) => return String::new(),
        },
        Data::Float(x) => *x,
        Data::DateTime(dt) => dt.as_f64() - 1.0,
        _ => return String::new(),
    };

    let start = NaiveDate::from_ymd_opt(1899, 12, 31).expect("Invalid date");
    let date_option = start.checked_add_signed(Duration::days(serial as i64));

    if let Some(date) = date_option {
        format!("{}/{}/{}", date.month(), date.day(), date.year())
    } else {
        String::new()
    }
}
