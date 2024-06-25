# Amazon Book Search Scraper

This Rust project scrapes search results from Amazon Japan for a specific book title and saves the results to a CSV file.

## Description

This script does the following:
1. Opens Amazon Japan's website
2. Searches for "The Brothers Karamazov"
3. Extracts information about each search result (title, author, price, and rating)
4. Saves this information to a CSV file

## Prerequisites

- Rust (latest stable version)
- ChromeDriver
- Chrome or Chromium browser

## Dependencies

This project uses the following crates:
- `thirtyfour`: For browser automation
- `tokio`: For asynchronous runtime
- `csv`: For writing CSV files