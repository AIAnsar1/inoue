use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use strum::EnumString;
use crate::support::Operation::Get;

#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long, conflicts_with = "scenario", required_unless_present = "scenario")]
    target: Option<String>,
    #[arg(short, long, conflicts_with = "scenario")]
    request_body: Option<String>,
    #[arg(short, long, default_value_t = 1, conflicts_with = "scenario")]
    clients: usize,
    #[arg(short, long, default_value_t = 1, conflicts_with_all = ["duration", "scenario"])]
    iterations: usize,
    #[arg(short, long, conflicts_with_all = ["iterations", "scenario"])]
    duration: Option<u64>,
    #[arg(long, conflicts_with = "scenario")]
    headers: Option<Vec<String>>,
    #[arg(long, conflicts_with = "target")]
    scenario: Option<String>,
}


#[derive(Eq, PartialEq, Debug, EnumString)]
pub enum Operation {
    #[strum(serialize = "GET")]
    Get,
    #[strum(serialize = "POST")]
    Post,
    Head,
    Patch,
    Put,
    Delete,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub clients: usize,
    pub requests: usize,
    pub target: String,
    pub keep_alive: Option<Duration>,
    pub body: Option<String>,
    pub headers: Option<Vec<Header>>,
    pub duration: Option<u64>,
    pub verbose: bool,
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub key: String,
    pub value: String,
}



/**
 *=================================================================
 * to_string()
 *=================================================================
 * Converts Args into a Settings instance.
 *
 * If no scenario is provided, it initializes Settings from Args.
 * If a file is provided, it initializes Settings from the file.
 *
 *=================================================================
 * @return Result<Settings>
 */
impl Args {
    pub fn ino_to_string(self) -> Result<Settings> {
        match self.scenario {
            None => Settings::ino_from_args(self),
            Some(file) => Settings::ino_from_file(file),
        }
    }
}



impl Settings {

    /**
    *=================================================================
    * ino_print_banner()
    *=================================================================
    *
    * Prints a banner with the settings summary.
    *
    * Displays either iteration-based or time-based
    * execution details.
    *=================================================================
    * @param void
    * @return void
    */
    pub fn ino_print_banner(&self) {
        let banner = match &self.duration {
            None => format!(
                "kamehameha to {} with {} concurrent clients and {} total iterations",
                &self.target, &self.clients, &self.requests
            ),
            Some(d) => format!(
                "kamehameha to {} with {} concurrent clients for {} seconds",
                &self.target, &self.clients, d
            ),
        };
        println!("{}", banner);
    }


    /**
    *=================================================================
    * ino_request_by_client()
    *=================================================================
    *
    * Calculates the number of requests per client.
    *
    * Divides total requests by the number of clients.
    *
    *
    *=================================================================
    * @param void
    * @return usize
    */
    pub fn ino_requests_by_client(&self) -> usize {
        self.requests / self.clients
    }


    /**
    *=================================================================
    * ino_from_file()
    *=================================================================
    *
    * Loads settings from a YAML file.
    *
    * Parses the file and returns a Settings instance.
    * Handles errors with appropriate messages.
    *
    *
    *=================================================================
    * @param file String
    * @return Result<Self>
    */
    pub fn ino_from_file(file: String) -> Result<Self> {
        let content = fs::read_to_string(&file)
            .with_context(|| format!("Failed to read file from {}", &file))?;
        let settings: Settings = serde_yaml::from_str(&content)
            .with_context(|| "Invalid YAML format".to_string())?;
        Ok(settings)
    }


    /**
    *=================================================================
    * ino_from_args()
    *=================================================================
    * Creates a Settings instance from Args.
    *
    * Handles headers and request body.
    * Populates Settings with provided arguments.
    *
    *
    *=================================================================
    *
    * @param args Args
    * @return Result<Self>
    *
    */
    pub fn ino_from_args(args: Args) -> Result<Self> {
        let headers = args.headers.map(|headers_string| {
            headers_string
                .iter()
                .filter_map(|header| {
                    let split: Vec<&str> = header.split(':').collect();
                    if split.len() == 2 {
                        Some(Header {
                            key: split[0].trim().to_string(),
                            value: split[1].trim().to_string(),
                        })
                    } else {
                        None
                    }
                })
                .collect()
        });

        let body = match args.request_body {
            None => None,
            Some(file) => {
                let content = fs::read_to_string(&file)
                    .with_context(|| format!("Failed to read file from {}", &file))?;
                Some(content)
            }
        };

        Ok(Settings {
            clients: args.clients,
            requests: args.iterations,
            target: args.target.expect("Target URL is required"),
            keep_alive: None,
            body,
            headers,
            duration: args.duration,
            verbose: args.verbose,
        })
    }


    /**
    *=================================================================
    * ino_operation()
    *=================================================================
    *
    * Determines the operation (e.g., HTTP method).
    *
    * Parses the target string and returns the corresponding operation.
    * Defaults to GET if unspecified.
    *
    *
    *=================================================================
    *
    * @param void
    * @return Operation
    *
    */
    pub fn ino_operation(&self) -> Operation {
        let slices: Vec<&str> = self.target.split_whitespace().collect();

        slices
            .first()
            .map(|op| Operation::from_str(&op.to_uppercase()).unwrap_or(Operation::Get))
            .unwrap_or(Operation::Get)
    }


    /**
    *=================================================================
    * ino_target()
    *=================================================================
    *
    * Extracts the URL target.
    *
    * Parses the target string and returns the URL component.
    *
    *=================================================================
    * @param void
    * @return String
    *
    */
    pub fn ino_target(&self) -> String {
        let slices: Vec<&str> = self.target.split_whitespace().collect();

        if slices.len() == 1 {
            slices
                .first()
                .expect("Target is not well formatted")
                .to_string()
        } else {
            slices
                .get(1)
                .expect("Target is not well formatted")
                .to_string()
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::support::Operation::{Get, Post};

    #[test]
    fn should_set_get_as_default_operation() -> Result<()> {
        let args = Args {
            target: Some("https://localhost:3000".to_string()),
            ..Default::default()
        };

        let settings = Settings::ino_from_args(args)?;
        assert_eq!(Get, settings.ino_operation());
        Ok(())
    }

    #[test]
    fn should_get_operation_from_target() -> Result<()> {
        let args = Args {
            target: Some("POST https://localhost:3000".to_string()),
            ..Default::default()
        };

        let settings = Settings::ino_from_args(args)?;
        assert_eq!(Post, settings.ino_operation());
        Ok(())
    }

    #[test]
    fn should_get_target_from_target_without_operation() -> Result<()> {
        let args = Args {
            target: Some("https://localhost:3000".to_string()),
            ..Default::default()
        };

        let settings = Settings::ino_from_args(args)?;
        assert_eq!("https://localhost:3000", settings.ino_target());
        Ok(())
    }

    #[test]
    fn should_get_target_from_target_with_operation() -> Result<()> {
        let args = Args {
            target: Some("POST https://localhost:3000".to_string()),
            ..Default::default()
        };

        let settings = Settings::ino_from_args(args)?;
        assert_eq!("https://localhost:3000", settings.ino_target());
        Ok(())
    }

    #[test]
    fn should_set_get_operation_if_operation_is_not_allowed() -> Result<()> {
        let args = Args {
            target: Some("FOO https://localhost:3000".to_string()),
            ..Default::default()
        };

        let settings = Settings::ino_from_args(args)?;
        assert_eq!(Get, settings.ino_operation());
        Ok(())
    }

    #[test]
    fn should_return_error_if_request_body_file_does_not_exists() -> Result<()> {
        let args = Args {
            target: Some("POST https://localhost:3000".to_string()),
            request_body: Some(String::from("foo")),
            ..Default::default()
        };
        match Settings::ino_from_args(args) {
            Ok(_) => {}
            Err(e) => {
                assert_eq!(e.to_string(), "Failed to read file from foo")
            }
        }
        Ok(())
    }

    #[test]
    fn should_set_none_headers_if_not_present() -> Result<()> {
        let args = Args {
            target: Some("FOO https://localhost:3000".to_string()),
            request_body: None,
            ..Default::default()
        };
        let settings = Settings::ino_from_args(args)?;
        assert_eq!(settings.headers, None);
        Ok(())
    }

    #[test]
    fn should_set_headers() -> Result<()> {
        let args = Args {
            target: Some("FOO https://localhost:3000".to_string()),
            headers: Some(vec![
                "bar:foo".to_string(),
                "Content-Type:application/json".to_string(),
            ]),
            ..Default::default()
        };
        let settings = Settings::ino_from_args(args)?;
        assert_eq!(
            settings.headers,
            Some(vec![
                Header {
                    key: "bar".to_string(),
                    value: "foo".to_string(),
                },
                Header {
                    key: "Content-Type".to_string(),
                    value: "application/json".to_string(),
                },
            ])
        );
        Ok(())
    }
}