use crate::{data::{INSTANCES, structure::{Instance, Template}, TEMPLATES}, error::PangError};

/// Pushes a [`Template`] to the static [`TEMPLATES`] mutex
pub fn push_template(template: Template, loc: usize) -> Result<(), PangError> {
    let mut mutex = TEMPLATES.lock().unwrap();
    let res: Vec<&Template> = mutex.iter().filter(|t| {
        t.name == template.name
    }).collect();
    if res.len() > 0 {
        return Err(PangError::TemplateAlreadyExists(res[0].name.clone(), loc))
    }

    mutex.push(template);
    Ok(())
}

/// Pushes a [`Instance`] to the static [`INSTANCES`] mutex
pub fn push_instance(instance: Instance, loc: usize) -> Result<(), PangError> {
    let mut mutex = INSTANCES.lock().unwrap();
    let res: Vec<&Instance> = mutex.iter().filter(|t| {
        t.name == instance.name
    }).collect();
    if res.len() > 0 {
        return Err(PangError::InstanceAlreadyExists(res[0].name.clone(), loc))
    }

    mutex.push(instance);
    Ok(())
}

/// Removes a [`Instance`] from the static [`INSTANCES`] mutex based on a name and returns the removed element
pub fn remove_instance(name: String, loc: usize) -> Result<Instance, PangError> {
    let mut mutex = INSTANCES.lock().unwrap();
    let mut index = 0;
    for (i, instance) in mutex.iter().enumerate() {
        if instance.name == name {
            index = i;
            break;
        }
        if mutex.len()-1 == i {
            return Err(PangError::InstanceNonExistent(name, loc))
        }
    }
    if mutex.len() > 1 {
        Ok(mutex.swap_remove(index))
    } else if mutex.len() != 0 {
        Ok(mutex.remove(index))
    } else {
        Err(PangError::InstanceNonExistent(name, loc))
    }
}

/// Removes a [`Template`] from the static [`TEMPLATES`] mutex based on a name and returns the removed element
pub fn remove_template(name: String, loc: usize) -> Result<Template, PangError> {
    let mut temp_mutex = TEMPLATES.lock().unwrap();
    let mut index = 0;
    for (i, template) in temp_mutex.iter().enumerate() {
        if template.name == name {
            index = i;
            break;
        }
        if temp_mutex.len()-1 == i {
            return Err(PangError::TemplateNonExistent(name, loc))
        }
    }
    // Remove template
    let temp = {
        if temp_mutex.len() > 1 {
            Ok(temp_mutex.swap_remove(index))
        } else if temp_mutex.len() != 0 {
            Ok(temp_mutex.remove(index))
        } else {
            Err(PangError::InstanceNonExistent(name, loc))
        }
    }?;

    // Remove instances
    let mut inst_mutex = INSTANCES.lock().unwrap();
    inst_mutex.drain_filter(|inst| {
        inst.template == temp
    });

    Ok(temp)
}

/// Copies a [`Instance`] from the static [`INSTANCES`] mutex based on a name and returns it
pub fn copy_instance(name: String, loc: usize) -> Result<Instance, PangError> {
    let mutex = INSTANCES.lock().unwrap();
    let mut index = 0;
    for (i, instance) in mutex.iter().enumerate() {
        if instance.name == name {
            index = i;
            break;
        }
        if mutex.len()-1 == i {
            return Err(PangError::InstanceNonExistent(name, loc))
        }
    }
    Ok(mutex.get(index).unwrap().clone())
}

/// Copies a [`Template`] from the static [`TEMPLATES`] mutex based on a name and returns it
pub fn copy_template(name: String, loc: usize) -> Result<Template, PangError> {
    let mutex = TEMPLATES.lock().unwrap();
    let mut index = 0;
    for (i, template) in mutex.iter().enumerate() {
        if template.name == name {
            index = i;
            break;
        }
        if mutex.len()-1 == i {
            return Err(PangError::TemplateNonExistent(name, loc))
        }
    }
    Ok(mutex.get(index).unwrap().clone())
}

/// Copies the whole [`TEMPLATES`] vec.
pub fn copy_templates() -> Vec<Template> {
    let mutex = TEMPLATES.lock().unwrap();
    return mutex.clone();
}

/// Copies the whole [`INSTANCES`] vec.
pub fn copy_instances() -> Vec<Instance> {
    let mutex = INSTANCES.lock().unwrap();
    return mutex.clone();
}