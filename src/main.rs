use std::{
  collections::HashMap, path::Path, sync::mpsc::channel, thread::JoinHandle, time::Duration,
};

use anyhow::Context;
use chrono::{DateTime, Local};
use configuration::{load_and_parse_yaml, Config};
use cron::Schedule;
use eskom_se_push_api::{status::LoadsheddingStatus, Endpoint};
use inquire::Confirm;
use std::str::FromStr;

use crate::{configuration::write_to_yaml, configuration_setup::configuration_process};
mod configuration;
mod configuration_setup;

fn main() -> anyhow::Result<()> {
  let c = {
    match load_and_parse_yaml::<Config>(Path::new("config.yaml")) {
      Ok(config) => config,
      Err(e) => {
        eprintln!(
          "An error occurred: {}. It could be because that the configuration is not done.",
          e
        );
        let ans = Confirm::new("Do you want to continue with the validation?")
          .with_default(false)
          .prompt()
          .context("Exiting")?;
        if ans {
          configuration_process()
        } else {
          return Ok(());
        }
      }
    }
  };
  write_to_yaml(Path::new("test.yaml"), &c)?;
  return Ok(());
  // TODO see if using channels would help or returning custom type with prompt
  let api = eskom_se_push_api::allowance::AllowanceCheckURLBuilder::default().build()?;
  let resp = api.ureq(&c.token)?;
  let local_timezone = Local::now().timezone();

  // Status API call
  let api_status_call_cron = get_cron_job_based_on_limit(resp.allowance.limit);
  let api_status_call_schedule = Schedule::from_str(api_status_call_cron).unwrap();
  let mut next_api_call_time = api_status_call_schedule
    .upcoming(local_timezone)
    .take(1)
    .next()
    .unwrap();

  // 55 minutes before the hour
  let notification_55_schedule = Schedule::from_str("5 * * * *").unwrap();
  let mut notification_55_time = notification_55_schedule
    .upcoming(local_timezone)
    .take(1)
    .next()
    .unwrap();

  // 15 minutes before the hour
  let notification_15_schedule = Schedule::from_str("45 * * * *").unwrap();
  let mut notification_15_time = notification_15_schedule
    .upcoming(local_timezone)
    .take(1)
    .next()
    .unwrap();

  let (tx, rx) = channel();
  ctrlc::set_handler(move || tx.send(true).expect("Could not send signal on channel."))
    .expect("Error setting Ctrl-C handler");

  println!("{:?}", resp);
  let token = c.token.clone();
  let mut api_thread: Option<
    JoinHandle<anyhow::Result<HashMap<std::string::String, LoadsheddingStatus>>>,
  > = None;
  loop {
    let local = Local::now();
    std::thread::sleep(Duration::from_secs(1));
    if let Some(t) = api_thread.take() {
      if t.is_finished() {
        match t.join() {
            Ok(resp) => todo!(),
            Err(e) => todo!(),
        }
        api_thread = None;
      } else {
        api_thread = Some(t);
      }
    }
    if next_api_call_time.timestamp() <= local.timestamp() {
      next_api_call_time = api_status_call_schedule
        .upcoming(local_timezone)
        .take(1)
        .next()
        .unwrap();
      api_thread = Some(std::thread::spawn(move || get_stage_status(&token.clone())));
      todo!("API call thread");
    }
    if notification_55_time.timestamp() <= local.timestamp() {
      // TODO change `next_api_call_time` to the actual loadshedding start time
      if about_55_min(&next_api_call_time, &local) {
        todo!("Send 55 min notification");
      }
      notification_55_time = notification_55_schedule
        .upcoming(local_timezone)
        .take(1)
        .next()
        .unwrap();
    }
    if notification_15_time.timestamp() <= local.timestamp() {
      // TODO change `next_api_call_time` to the actual loadshedding start time
      if about_5_min(&next_api_call_time, &local) {
        todo!("Send 5 min notification");
      }
      notification_15_time = notification_15_schedule
        .upcoming(local_timezone)
        .take(1)
        .next()
        .unwrap();
    }
    if let Ok(v) = rx.try_recv() {
      if v {
        break;
      }
    }
  }
  Ok(())
}

// /**
//  * 50 Daily
//  *  Every 30 min average
// */30 * * * * // Every 30 minutes
//  * 200 Daily
//  *  Every 7 min average
// */7 * * * * // Every 7 minutes
//  * 2500 Daily
//  *  Every 0.6 min average
// * * * * * // Every minute
//  */

/// Returns a cron string based on the limit. The assumption that all the
/// API calls will be used to check the national status.
fn get_cron_job_based_on_limit(limit: i64) -> &'static str {
  match limit {
    50 => "*/30 * * * *", // Every 30 minutes
    200 => "*/7 * * * *", // Every 7 minutes,
    _ => "* * * * *",     // Every minute,
  }
}

// Return stage
fn get_stage_status(
  token: &str,
) -> anyhow::Result<HashMap<std::string::String, LoadsheddingStatus>> {
  let api = eskom_se_push_api::status::EskomStatusUrlBuilder::default()
    .build()
    .unwrap();
  match api.ureq(&token.clone()) {
    Ok(status_response) => {
      Ok(status_response.status)
      // todo!("Check if status is different, time slots etc");
    }
    Err(e) => match e {
      eskom_se_push_api::errors::HttpError::APIError(err_api) => match err_api {
        eskom_se_push_api::errors::APIError::BadRequest => Err(anyhow::anyhow!("Bad API request")),
        eskom_se_push_api::errors::APIError::Forbidden => {
          Err(anyhow::anyhow!("Token is not valid"))
        }
        eskom_se_push_api::errors::APIError::NotFound => Err(anyhow::anyhow!("Unknown endpoint")),
        eskom_se_push_api::errors::APIError::TooManyRequests => {
          Err(anyhow::anyhow!("Have reached API daily limit"))
        }
        eskom_se_push_api::errors::APIError::ServerError(e) => {
          Err(anyhow::anyhow!("Server error: {}", e))
        }
      },
      eskom_se_push_api::errors::HttpError::Timeout => Err(anyhow::anyhow!("API call timed out")),
      eskom_se_push_api::errors::HttpError::NoInternet => {
        Err(anyhow::anyhow!("No Internet access"))
      }
      eskom_se_push_api::errors::HttpError::Unknown => Err(anyhow::anyhow!("Unknown error")),
      eskom_se_push_api::errors::HttpError::UnknownError(e) => {
        Err(anyhow::anyhow!("Unknown error: {}", e))
      }
      eskom_se_push_api::errors::HttpError::ResponseError(_)
      | eskom_se_push_api::errors::HttpError::UreqResponseError(_)
      | eskom_se_push_api::errors::HttpError::SearchTextNotSet
      | eskom_se_push_api::errors::HttpError::AreaIdNotSet => {
        Err(anyhow::anyhow!("Should not reach here"))
      }
      eskom_se_push_api::errors::HttpError::LongitudeOrLatitudeNotSet {
        longitude: _,
        latitude: _,
      } => Err(anyhow::anyhow!("Should not reach here")),
    },
  }
}

/// Checks if there is about 55 minutes between the 2 times
fn about_55_min(next: &DateTime<Local>, current: &DateTime<Local>) -> bool {
  let t = next.time() - current.time();
  let duration = t.num_seconds();
  // Checking if between 55 min and 54:45 min
  duration <= 55 * 60 && duration >= 54 * 60 + 45
}

/// Checks if there is about 5 minutes between the 2 times
fn about_5_min(next: &DateTime<Local>, current: &DateTime<Local>) -> bool {
  let t = next.time() - current.time();
  let duration = t.num_seconds();
  // Checking if between 5 min and 4:45 min
  duration <= 5 * 60 && duration >= 4 * 60 + 45
}

/// Checks if there is about 15 minutes between the 2 times
fn about_15_min(next: &DateTime<Local>, current: &DateTime<Local>) -> bool {
  let t = next.time() - current.time();
  let duration = t.num_seconds();
  // Checking if between 15 min and 14:45 min
  duration <= 15 * 60 && duration >= 14 * 60 + 45
}

/// Checks if there is about 15 minutes between the 2 times
fn about_x_min(minutes: i64, next: &DateTime<Local>, current: &DateTime<Local>) -> bool {
  if minutes < 1 {
    panic!("Minutes must be 1 or greater");
  }
  let t = next.time() - current.time();
  let duration = t.num_seconds();
  // Checking if between {minutes} min and {minutes - 1}:45 min
  duration <= minutes * 60 && duration >= minutes * 60 - 15
}
