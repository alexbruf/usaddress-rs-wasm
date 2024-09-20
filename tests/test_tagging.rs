// use core::panic;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

use usaddress_rs_wasm::usaddr::parse;
use xml::reader::XmlEvent;
use xml::EventReader;

enum Method {
    Exact,
    Fuzzy,
}

pub fn read_xml_tagged_addresses(file_path: &str) -> (Vec<String>, Vec<Vec<String>>) {
    let file = File::open(file_path);
    let file = match file {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            std::process::exit(1);
        }
    };
    let file = BufReader::new(file);

    let parser = EventReader::new(file);

    let mut addresses: Vec<String> = Vec::new();
    let mut tags: Vec<Vec<String>> = Vec::new();

    let mut address: Vec<String> = Vec::new();
    let mut yseq: Vec<String> = Vec::new();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name.local_name == "AddressString" {
                    address.clear();
                    yseq.clear();
                } else {
                    yseq.push(name.local_name.to_string());
                }
            }
            Ok(XmlEvent::Characters(s)) => {
                address.push(s);
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "AddressString" {
                    addresses.push(address.join(" "));
                    tags.push(yseq.clone());
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            _ => {}
        }
    }

    (addresses, tags)
}

fn get_erroring_addresses(
    file_path: &str,
    method: Method,
) -> Result<(Vec<String>, u32, HashSet<usize>, usize), std::io::Error> {
    let mut errors = Vec::new();
    let mut n_tags = 0;
    let mut failed_addresses = HashSet::new();

    let (addresses, tags) = read_xml_tagged_addresses(file_path);
    for (i, address) in addresses.iter().enumerate() {
        let parsed = parse(address)?;
        let mut parsed_tags = parsed.iter().map(|x| x.1.clone()).collect::<Vec<String>>();

        if let Method::Fuzzy = method {
            parsed_tags = get_fuzzy_labels(&parsed_tags);
        }

        if parsed.len() != tags[i].len() {
            errors.push(format!(
                "{}. Mismatched length: {} != {}",
                i,
                parsed.len(),
                tags[i].len()
            ));
        }
        for (j, tag) in tags[i].iter().enumerate() {
            n_tags += 1;
            // assert_eq!(parsed[j].1, tag.to_string());
            if parsed_tags[j] != tag.to_string() {
                // errors.push((i, address.to_string(), parsed.clone(), tags[i].to_vec()));
                // errors.push((i, address.to_string(), parsed[j].1.clone(), tag.to_string()));
                errors.push(format!(
                    "{}. Mismatched tag: {} != {}",
                    i,
                    parsed_tags[j],
                    tag.to_string()
                ));
                failed_addresses.insert(i);
            }
        }
    }
    Ok((errors, n_tags, failed_addresses, addresses.len()))
}

fn get_fuzzy_labels(labels: &Vec<String>) -> Vec<String> {
    let mut fuzzy_labels = Vec::new();

    for label in labels.iter() {
        if label.starts_with("StreetName") {
            fuzzy_labels.push("StreetName".to_string());
        } else if label.starts_with("AddressNumber") {
            fuzzy_labels.push("AddressNumber".to_string());
        } else if *label == "Null" {
            fuzzy_labels.push("NotAddress".to_string());
        } else {
            fuzzy_labels.push(label.clone());
        }
    }

    fuzzy_labels
}

#[test]
fn test_simple_address_patterns() {
    let (errors, n_tags, failed_addresses, n_addresses) =
        get_erroring_addresses("tests/test_data/simple_address_patterns.xml", Method::Exact).unwrap();

    if !errors.is_empty() {
        println!(
            "There were {} mistagged address components of {} ({:.1}%). {} partially failed addresses of {} ({:.1}%)",
            errors.len(),
            n_tags,
            errors.len() as f64 / n_tags as f64 * 100.0,
            failed_addresses.len(),
            n_addresses,
            failed_addresses.len() as f64 / n_addresses as f64 * 100.0,
        );
    }
}

#[test]
fn test_labeled() {
    let (errors, n_tags, failed_addresses, n_addresses) =
        get_erroring_addresses("tests/test_data/labeled.xml", Method::Exact).unwrap();

    if !errors.is_empty() {
        println!(
            "There were {} mistagged address components of {} ({:.1}%). {} partially failed addresses of {} ({:.1}%)",
            errors.len(),
            n_tags,
            errors.len() as f64 / n_tags as f64 * 100.0,
            failed_addresses.len(),
            n_addresses,
            failed_addresses.len() as f64 / n_addresses as f64 * 100.0,
        );
    }
}

#[test]
fn test_us50_tagged() {
    let (errors, n_tags, failed_addresses, n_addresses) =
        get_erroring_addresses("tests/test_data/us50_test_tagged.xml", Method::Fuzzy).unwrap();

    if !errors.is_empty() {
        println!(
            "There were {} mistagged address components of {} ({:.1}%). {} partially failed addresses of {} ({:.1}%)",
            errors.len(),
            n_tags,
            errors.len() as f64 / n_tags as f64 * 100.0,
            failed_addresses.len(),
            n_addresses,
            failed_addresses.len() as f64 / n_addresses as f64 * 100.0,
        );
    }
}

#[test]
fn test_synthetic_clean_osm_data() {
    let (errors, n_tags, failed_addresses, n_addresses) = get_erroring_addresses(
        "tests/test_data/synthetic_clean_osm_data.xml",
        Method::Exact,
    ).unwrap();

    if !errors.is_empty() {
        println!(
            "There were {} mistagged address components of {} ({:.1}%). {} partially failed addresses of {} ({:.1}%)",
            errors.len(),
            n_tags,
            errors.len() as f64 / n_tags as f64 * 100.0,
            failed_addresses.len(),
            n_addresses,
            failed_addresses.len() as f64 / n_addresses as f64 * 100.0,
        );
    }
}
