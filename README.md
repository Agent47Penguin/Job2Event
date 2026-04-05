# Job2Event

Converts a row from a job scheduling spreadsheet into a Google Calendar event. Opens a local web UI, pre-fills the event details from the Excel file, and launches Google Calendar when you're ready.

## How It Works

1. Run the app with your Excel file and row number
2. A browser window opens at `http://localhost:8080`
3. Review and edit the pre-filled event details
4. Click **Add to Google Calendar** — the app closes and opens Calendar with everything filled in

## Usage

```
job2event.exe <path-to-xlsx> <row-number>
```

**Example:**
```
job2event.exe "C:\Jobs\schedule.xlsx" 14
```

If no file is provided the UI opens with empty fields you can fill in manually.

## Excel Column Layout

The app expects a specific column order in your spreadsheet:

| Column | Field |
|--------|-------|
| A | Date Contacted |
| B | Task |
| C | Same Day |
| D | Scheduled |
| E | Travel |
| F | Job Name |
| G | Address |
| H | Billing First Name |
| I | Billing Last Name |
| J | Billing Company |
| K | Phone Number |
| L | On-Site Contact |
| M | On-Site Number |
| N | GC Name |
| O | GC Number |
| P | Permit Number |
| Q | PO Number |
| R | Water Purveyor |
| S | Due Date |
| T | Coordination |
| U | Parts |
| V | Notes |
| W | Calendar Entry (event title) |

## Building from Source

Requires Rust (2024 edition).

```
cargo build --release
```

The binary will be at `target/release/job2event.exe`.

## Releases

Windows x64 binaries are available on the [Releases](../../releases) page. Download the zip, extract `job2event.exe`, and run it from the command line or via a macro/shortcut.

## License

Apache 2.0 with Commons Clause — free for personal and internal business use. Commercial use requires permission. See [LICENSE.txt](LICENSE.txt) for details.
