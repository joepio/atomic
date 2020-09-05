//! Creating a new resource. Provides prompting logic
use atomic_lib::urls;
use atomic_lib::{storelike::{Property, Class}, values::DataType, errors::AtomicResult, Resource, Value, Storelike};
use atomic_lib::mapping;
use crate::Context;
use std::time::{UNIX_EPOCH, SystemTime};
use colored::Colorize;
use promptly::prompt_opt;
use regex::Regex;

/// Create a new instance of some class through a series of prompts
pub fn new(context: &mut Context) {
  let class_input = context
      .matches
      .subcommand_matches("new")
      .unwrap()
      .value_of("class")
      .expect("Add a class value");

  let class_url = context
      .mapping
      .try_mapping_or_url(&class_input.into())
      .unwrap();
  let model = context.store.get_class(&class_url);
  println!("Enter a new {}: {}", model.shortname, model.description);
  prompt_instance(context, &model, None).unwrap();
}

/// Lets the user enter an instance of an Atomic Class through multiple prompts
/// Adds the instance to the store, and writes to disk.
/// Returns the Resource, its URL and its Bookmark.
fn prompt_instance(
  context: &mut Context,
  class: &Class,
  preffered_shortname: Option<String>,
) -> AtomicResult<(Resource, String, Option<String>)> {
  // Not sure about the best way t
  // The Path is the thing at the end of the URL, from the domain
  // Here I set some (kind of) random numbers.
  // I think URL generation could be better, though. Perhaps use a
  let path = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .subsec_nanos();

  let mut subject = format!("_:{}", path);
  if preffered_shortname.is_some() {
      subject = format!("_:{}-{}", path, preffered_shortname.clone().unwrap());
  }

  let mut new_resource: Resource = Resource::new(subject.clone());

  new_resource.insert(
      "https://atomicdata.dev/properties/isA".into(),
      Value::ResourceArray(Vec::from([class.subject.clone().into()])),
  )?;

  for field in &class.requires {
      if field.subject == atomic_lib::urls::SHORTNAME && preffered_shortname.clone().is_some() {
          new_resource.insert_string(
              field.subject.clone(),
              &preffered_shortname.clone().unwrap(),
              &mut context.store,
          )?;
          println!(
              "Shortname set to {}",
              preffered_shortname.clone().unwrap().bold().green()
          );
          continue;
      }
      println!("{}: {}", field.shortname.bold().blue(), field.description);
      // In multiple Properties, the shortname field is required.
      // A preferred shortname can be passed into this function
      let mut input = prompt_field(&field, false, context)?;
      loop {
          if let Some(i) = input {
              new_resource.insert_string(field.subject.clone(), &i, &mut context.store)?;
              break;
          } else {
              println!("Required field, please enter a value.");
              input = prompt_field(&field, false, context)?;
          }
      }
  }

  for field in &class.recommends {
      println!("{}: {}", field.shortname.bold().blue(), field.description);
      let input = prompt_field(&field, true, context)?;
      if let Some(i) = input {
          new_resource.insert_string(field.subject.clone(), &i, &mut context.store)?;
      }
  }

  println!(
      "{} created with URL: {}",
      &class.shortname,
      &subject.clone()
  );

  let map = prompt_bookmark(&mut context.mapping, &subject);

  // Add created_instance to store
  context
      .store
      .add_resource_string(new_resource.subject().clone(), &new_resource.to_plain())
      .unwrap();
  context
      .mapping
      .write_mapping_to_disk(&context.user_mapping_path);
  return Ok((new_resource, subject, map));
}

// Checks the property and its datatype, and issues a prompt that performs validation.
fn prompt_field(
  property: &Property,
  optional: bool,
  context: &mut Context,
) -> AtomicResult<Option<String>> {
  let mut input: Option<String> = None;
  let msg_appendix;
  if optional {
      msg_appendix = " (optional)";
  } else {
      msg_appendix = " (required)";
  }
  match &property.data_type {
      DataType::String | DataType::Markdown => {
          let msg = format!("string{}", msg_appendix);
          input = prompt_opt(&msg).unwrap();
          return Ok(input);
      }
      DataType::Slug => {
          let msg = format!("slug{}", msg_appendix);
          input = prompt_opt(&msg).unwrap();
          let re = Regex::new(atomic_lib::values::SLUG_REGEX).unwrap();
          match input {
              Some(slug) => {
                  if re.is_match(&*slug) {
                      return Ok(Some(slug));
                  }
                  println!("Only letters, numbers and dashes - no spaces or special characters.");
                  return Ok(None);
              }
              None => (return Ok(None)),
          }
      }
      DataType::Integer => {
          let msg = format!("integer{}", msg_appendix);
          let number: Option<u32> = prompt_opt(&msg).unwrap();
          match number {
              Some(nr) => {
                  input = Some(nr.to_string());
              }
              None => (return Ok(None)),
          }
      }
      DataType::Date => {
          let msg = format!("date YYYY-MM-DD{}", msg_appendix);
          let date: Option<String> = prompt_opt(&msg).unwrap();
          let re = Regex::new(atomic_lib::values::DATE_REGEX).unwrap();
          match date {
              Some(date_val) => loop {
                  if re.is_match(&*date_val) {
                      return Ok(Some(date_val));
                  }
                  println!("Not a valid date.");
                  return Ok(None)
              },
              None => (return Ok(None)),
          }
      }
      DataType::AtomicUrl => loop {
          let msg = format!("URL{}", msg_appendix);
          let url: Option<String> = prompt_opt(msg).unwrap();
          // If a classtype is present, the given URL must be an instance of that Class
          let classtype = &property.class_type;
          if classtype.is_some() {
              let class = context
                  .store
                  .get_class(&String::from(classtype.as_ref().unwrap()));
              println!("Enter the URL or shortname of a {}", class.description)
          }
          match url {
              Some(u) => {
                  // TODO: Check if string or if map
                  input = context.mapping.try_mapping_or_url(&u);
                  match input {
                      Some(url) => return Ok(Some(url)),
                      None => {
                          println!("Shortname not found, try again.");
                          return Ok(None);
                      }
                  }
              }
              None => (),
          };
      },
      DataType::ResourceArray => loop {
          let msg = format!(
              "resource array - Add the URLs or Shortnames, separated by spacebars{}",
              msg_appendix
          );
          let option_string: Option<String> = prompt_opt(msg).unwrap();
          match option_string {
              Some(string) => {
                  let string_items = string.split(" ");
                  let mut urls: Vec<String> = Vec::new();
                  let length = string_items.clone().count();
                  for item in string_items.into_iter() {
                      match context.mapping.try_mapping_or_url(&item.into()) {
                          Some(url) => {
                              urls.push(url);
                          }
                          None => {
                              println!("Define the Property named {}", item.bold().green(),);
                              // TODO: This currently creates Property instances, but this should depend on the class!
                              let (_resource, url, _shortname) = prompt_instance(
                                  context,
                                  &context.store.get_class(&urls::PROPERTY.into()),
                                  Some(item.into()),
                              )?;
                              urls.push(url);
                              continue;
                          }
                      }
                  }
                  if length == urls.len() {
                      input = Some(atomic_lib::serialize::serialize_json_array(&urls).unwrap());
                      break;
                  }
              }
              None => break,
          }
      },
      DataType::Timestamp => todo!(),
      DataType::Unsupported(unsup) => panic!("Unsupported datatype: {:?}", unsup),
  };
  return Ok(input);
}

// Asks for and saves the bookmark. Returns the shortname.
fn prompt_bookmark(mapping: &mut mapping::Mapping, subject: &String) -> Option<String> {
  let re = Regex::new(atomic_lib::values::SLUG_REGEX).unwrap();
  let mut shortname: Option<String> = prompt_opt(format!("Local Bookmark (optional)")).unwrap();
  loop {
      match shortname.as_ref() {
          Some(sn) => {
              if mapping.contains_key(sn) {
                  let msg = format!(
                      "You're already using that shortname for {:?}, try something else",
                      mapping.get(sn).unwrap()
                  );
                  shortname = prompt_opt(msg).unwrap();
              } else if re.is_match(&sn.as_str()) {
                  &mut mapping.insert(String::from(sn), String::from(subject));
                  return Some(String::from(sn));
              } else {
                  shortname =
                      prompt_opt("Not a valid bookmark, only use letters, numbers, and '-'")
                          .unwrap();
              }
          }
          None => return None,
      }
  }
}