use crate::{Resource, Storelike, Value, agents::Agent, errors::AtomicResult, url_helpers::check_valid_url, urls};

/// If there is a valid Agent in the correct query param, and the invite is valid, update the rights and respond with a redirect to the target resource
pub fn construct_invite_redirect(
    store: &impl Storelike,
    query_params: url::form_urlencoded::Parse,
    invite_resource: &mut Resource,
    subject: &str
) -> AtomicResult<Resource> {
    let mut pub_key = None;
    let mut invite_agent = None;
    for (k, v) in query_params {
        match k.as_ref() {
            "public-key" | urls::INVITE_PUBKEY=> { pub_key = Some(v.to_string()) },
            "agent" | urls::AGENT=> { invite_agent = Some(v.to_string()) },
            _ => {}
        }
    }

    // Check if there is either a publicKey or an Agent present in the request. Either one is needed to continue accepting the invite.
    let agent = match (pub_key, invite_agent) {
        (None, None) => return Ok(invite_resource.to_owned()),
        (None, Some(agent_url)) => agent_url,
        (Some(public_key), None) => {
            let new_agent = Agent::new_from_public_key(store, &public_key)?;
            store.add_resource(&new_agent.to_resource(store)?)?;
            // I'd rather do this, but this errors:
            // new_agent.to_resource(store)?.save_locally(store)?;

            // Always add write rights to the agent itself
            // A bit inefficient, since it re-fetches the agent from the store, but it's not that big of a cost
            add_rights(&new_agent.subject, &new_agent.subject, true, store)?;
            new_agent.subject
        },
        (Some(_), Some(_)) => return Err("Either publicKey or agent can be set - not both at the same time.".into()),
    };

    // If there are write or read rights
    let write = if let Ok(bool) = invite_resource.get(urls::WRITE_BOOL){
        bool.to_bool()?
    } else {
        false
    };

    let target=  &invite_resource.get(urls::TARGET).map_err(
        |e| format!("Invite {} does not have a target. {}", invite_resource.get_subject(), e)
    )?.to_string();

    if let Ok(usages_left) = invite_resource.get(urls::USAGES_LEFT) {
        let num  = usages_left.to_int()?;
        if num == 0 {
            return Err("No usages left for this invite".into())
        }
        invite_resource.set_propval(urls::USAGES_LEFT.into(), Value::Integer(num - 1), store)?;
        invite_resource.save_locally(store).map_err(|e| format!("Unable to save updated Invite. {}", e))?;
    }

    // TODO: implement rights check
    // check_if_invite_is_valid(invite_resource)?;
    add_rights(&agent, target, write, store)?;

    let mut redirect = Resource::new_instance(urls::REDIRECT, store)?;
    redirect.set_propval(urls::DESTINATION.into(), invite_resource.get(urls::TARGET)?.to_owned(), store)?;
    redirect.set_propval(urls::REDIRECT_AGENT.into(), crate::Value::AtomicUrl(agent), store)?;
    // The front-end requires the @id to be the same as requested
    redirect.set_subject(subject.into());
    Ok(redirect)
}

/// Adds the requested rights to the target resource.
/// Overwrites the target resource to include the new rights.
/// Checks if the Agent has a valid URL.
/// Will not throw an error if the Agent already has the rights.
pub fn add_rights(agent: &str, target: &str, write: bool, store: &impl Storelike) -> AtomicResult<()> {
    check_valid_url(agent)?;
    // Get the Resource that the user is being invited to
    let mut target = store.get_resource(target)?;
    let right = if write {urls::WRITE} else {urls::READ};
    let mut rights_vector: Vec<String> = match target.get(right) {
        // Rights have been set, add to the list
        Ok(val) => {
            let vec = val.to_vec().map_err(|_| "Invalid value for rights")?;
            // If the vector already contains the agent, throw an error;
            for a in vec {
               if a == agent {
                   return Ok(())
               }
            }
            vec.to_owned()
        },
        // No rights have been set, create a new vector
        Err(_) => Vec::new(),
    };

    rights_vector.push(agent.to_string());

    target.set_propval(right.into(), rights_vector.into(), store)?;
    target.save_locally(store).map_err(|e| format!("Unable to save updated target resource. {}", e))?;

    Ok(())
}
