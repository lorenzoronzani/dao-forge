use std::collections::HashMap;

#[derive(Default)]
pub struct SogcPublicationTemplateManager {
    pub templates: HashMap<String, String>,
}

impl SogcPublicationTemplateManager {
    pub fn new() -> Self {
        let mut manager = Self::default();
        manager.load_default_templates();
        manager
    }

    fn load_default_templates(&mut self) {
        self.templates
            .insert("dao_created".to_string(), DAO_CREATED_TEMPLATE.to_string());

        self.templates.insert(
            "dao_name_changed".to_string(),
            DAO_NAME_CHANGED_TEMPLATE.to_string(),
        );

        self.templates.insert(
            "dao_member_added".to_string(),
            DAO_MEMBER_ADDED_TEMPLATE.to_string(),
        );

        self.templates.insert(
            "dao_member_removed".to_string(),
            DAO_MEMBER_REMOVED_TEMPLATE.to_string(),
        );

        self.templates.insert(
            "dao_address_updated".to_string(),
            DAO_ADDRESS_UPDATED_TEMPLATE.to_string(),
        );
    }

    pub fn render(
        &self,
        template_name: &str,
        data: HashMap<String, String>,
    ) -> Result<String, String> {
        let template = self
            .templates
            .get(template_name)
            .ok_or("Template not found")?;

        let mut rendered = template.clone();

        for (key, value) in data {
            rendered = rendered.replace(&format!("{{{}}}", key), &value);
        }

        Ok(rendered)
    }
}

const DAO_CREATED_TEMPLATE: &str = r#"COMMERCIAL REGISTER INSCRIPTION
                
By decision of the commercial registrar dated {date}, the following association has been inscribed in the commercial register:

Company name: {name}
Registered office: {address}, {zip} {town}
UID: {uid}
Commercial Registry No: {ch_id}
Federal Registry No: {frc_id}

Purpose: {purpose}

Management body:
{board}

Founding members:
{members}

This inscription takes effect from the aforementioned date. Constitutive documents are deposited with the registry office and may be consulted by any person with legitimate interest.

Published in the Swiss Official Gazette of Commerce (SOGC) in accordance with applicable legal provisions."#;

const DAO_NAME_CHANGED_TEMPLATE: &str = r#"COMMERCIAL REGISTER MODIFICATION

By decision of the commercial registrar dated {date}, the following modification has been inscribed in the commercial register:

Company name: {old_name}
NEW Company name: {new_name}
Registered office: {address}, {zip} {town}
UID: {uid}
Commercial Registry No: {ch_id}
Federal Registry No: {frc_id}

Modification: Change of company name from "{old_name}" to "{new_name}"

This modification takes effect from the aforementioned date. Updated constitutive documents are deposited with the registry office and may be consulted by any person with legitimate interest.

Published in the Swiss Official Gazette of Commerce (SOGC) in accordance with applicable legal provisions."#;

const DAO_MEMBER_ADDED_TEMPLATE: &str = r#"COMMERCIAL REGISTER MODIFICATION

By decision of the commercial registrar dated {date}, the following modification has been inscribed in the commercial register:

Company name: {name}
Registered office: {address}, {zip} {town}
UID: {uid}
Commercial Registry No: {ch_id}
Federal Registry No: {frc_id}

Modification: Addition of new member {new_member} with role {member_role}

This modification takes effect from the aforementioned date. Updated constitutive documents are deposited with the registry office and may be consulted by any person with legitimate interest.

Published in the Swiss Official Gazette of Commerce (SOGC) in accordance with applicable legal provisions."#;

const DAO_MEMBER_REMOVED_TEMPLATE: &str = r#"COMMERCIAL REGISTER MODIFICATION

By decision of the commercial registrar dated {date}, the following modification has been inscribed in the commercial register:

Company name: {name}
Registered office: {address}, {zip} {town}
UID: {uid}
Commercial Registry No: {ch_id}
Federal Registry No: {frc_id}

Modification: Removal of member {removed_member} with role {member_role}

This modification takes effect from the aforementioned date. Updated constitutive documents are deposited with the registry office and may be consulted by any person with legitimate interest.

Published in the Swiss Official Gazette of Commerce (SOGC) in accordance with applicable legal provisions."#;

const DAO_ADDRESS_UPDATED_TEMPLATE: &str = r#"COMMERCIAL REGISTER MODIFICATION

By decision of the commercial registrar dated {date}, the following modification has been inscribed in the commercial register:

Company name: {name}
OLD Registered office: {old_address}, {old_zip} {old_town}
NEW Registered office: {new_address}, {new_zip} {new_town}
UID: {uid}
Commercial Registry No: {ch_id}
Federal Registry No: {frc_id}

Modification: Change of registered office from "{old_address}, {old_zip} {old_town}" to "{new_address}, {new_zip} {new_town}"

This modification takes effect from the aforementioned date. Updated constitutive documents are deposited with the registry office and may be consulted by any person with legitimate interest.

Published in the Swiss Official Gazette of Commerce (SOGC) in accordance with applicable legal provisions."#;

#[cfg(test)]
#[path = "sogc_publication_template_tests.rs"]
mod sogc_publication_template_tests;
