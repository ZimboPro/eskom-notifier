use std::{error::Error, sync::mpsc::channel};

use eskom_se_push_api::{area_search::AreaSearch, Endpoint};
use inquire::{validator::Validation, Confirm, Select, Text};

use crate::configuration::{AreaInformation, Config};

fn select_area(response: &AreaSearch, api_token: &str) -> Option<AreaInformation> {
  let mut areas: Vec<String> = response
    .areas
    .iter()
    .map(|x| format!("{} - {}", x.region, x.name))
    .collect();
  let not_here = "None of the above";
  areas.push(not_here.clone().into());

  let selection = Select::new("Please select your area", areas.clone()).prompt();
  match selection {
    Ok(selected) => {
      if selected == not_here {
        return None;
      }
      let area = areas.iter().position(|x| *x == selected).unwrap();
      let area = response.areas.get(area).unwrap();
      let area_api = eskom_se_push_api::area_info::AreaInfoURLBuilder::default()
        .area_id(area.id.to_owned())
        .build()
        .unwrap();
      let area_info_response = area_api.ureq(api_token);
      match area_info_response {
        Ok(info) => {
          return Some(AreaInformation {
            area_id: area.id.clone(),
            info,
          });
        }
        Err(_) => {
          panic!("An error occurred while getting the area information that you selected")
        }
      };
    }
    Err(_) => panic!("An error occurred while making a selection"),
  }
}

fn get_area(token: &str) -> Option<AreaInformation> {
  let (tx, rx) = channel();
  let api_token = token.to_string().clone();
  let area_validator = move |input: &str| {
    let trimmed = input.trim();
    if trimmed.len() > 1 {
      let api = eskom_se_push_api::area_search::AreaSearchURLBuilder::default()
        .search_term(trimmed)
        .build()
        .unwrap();
      let resp = api.ureq(api_token.clone().as_str());
      match resp {
        Ok(response) => {
          if let Some(area_info) = select_area(&response, &api_token) {
            let _ = tx.send(area_info);
            return Ok(Validation::Valid);
          } else {
            return Ok(Validation::Invalid(
              "It seems your area was not in the list. Please type in another search term.".into(),
            ));
          }
        }
        Err(e) => Ok(Validation::Invalid(
          format!("An error occurred while getting a list of the similar areas: {e}").into(),
        )),
      }
    } else {
      Ok(Validation::Invalid(
        format!("The area {trimmed} is empty. It needs to have a value").into(),
      ))
    }
  };
  let _ans = Text::new("Please enter the name of the area you want to monitor.\n(This will contribute to the used limit so please be as accurate as possible)")
    .with_validator(area_validator)
    .prompt();
  match rx.recv() {
    Ok(area) => Some(area),
    Err(_) => None,
  }
}

fn token_validator(input: &str) -> Result<Validation, Box<dyn Error + Send + Sync>> {
  if input.trim().len() > 5 {
    let api = eskom_se_push_api::allowance::AllowanceCheckURLBuilder::default()
      .build()
      .unwrap();
    let resp = api.ureq(input);
    match resp {
      Ok(_) => Ok(Validation::Valid),
      Err(_) => Ok(Validation::Invalid(format!("An error occured while testing the token. Please test again later or check if you have internet access").into())),
    }
  } else {
    Ok(Validation::Invalid(
      format!("The token '{input}' is invalid").into(),
    ))
  }
}

pub fn configuration_process() -> Config {
  let token = Text::new(
    r#"Please enter your Eskom-Se-Push API key:
  (You can get a key at https://eskomsepush.gumroad.com/l/api )"#,
  )
  .with_validator(token_validator)
  .prompt();
  match token {
    Ok(api_token) => {
      let mut c = Config {
        token: api_token,
        areas: Vec::new(),
      };
      // Get at least one area
      if let Some(info) = get_area(&c.token) {
        c.areas.push(info);
      }
      loop {
        let t = Confirm::new("Do you want to continue with the validation?")
          .with_default(false)
          .prompt();
        match t {
          Ok(true) => {
            if let Some(info) = get_area(&c.token) {
              c.areas.push(info);
            }
          }
          _ => break,
        }
      }
      c
    }
    Err(_) => panic!("An error occurred in the prompt. Please try again."),
  }
}
