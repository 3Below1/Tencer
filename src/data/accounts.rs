use super::*;
use rand::prelude::*;

#[derive(Queryable)]
#[allow(dead_code)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub displayname: String,
    pub xp: i32,
    pub created_at: chrono::NaiveDateTime,
}

impl TencerData {
    pub fn get_account(&self, id: i32) -> QueryResult<Option<Account>> {
        use schema::*;

        accounts::table.find(id)
            .select(
                (accounts::id,
                accounts::username,
                accounts::displayname,
                accounts::xp,
                accounts::created_at))
            .first(&self.0).optional()
    }

    pub fn get_linked_accounts(&self, platform: &str, platform_id: &str) -> QueryResult<Vec<Account>> {
        use schema::*;

        let query = platform_id_links::table
            .filter(platform_id_links::platform.eq(platform))
            .filter(platform_id_links::platform_id.eq(platform_id))
            .select(platform_id_links::account_id)
            .inner_join(accounts::table)
            .select((accounts::id, accounts::username, accounts::displayname, accounts::xp, accounts::created_at));

        query.load(&self.0)
    }

    pub fn check_account_linked(&self, platform: &str, platform_id: &str, account_id: i32) -> QueryResult<bool> {
        use schema::*;

        let query = platform_id_links::table
            .filter(platform_id_links::platform.eq(platform))
            .filter(platform_id_links::platform_id.eq(platform_id))
            .filter(platform_id_links::account_id.eq(account_id));

        match query.first::<(i32, String, String, i32)>(&self.0) {
            Ok(_) => Ok(true),
            Err(e) => {
                match e {
                    diesel::result::Error::NotFound => Ok(false),
                    _ => Err(e),
                }
            }
        }

    }

    pub fn generate_account(&self, platform: &str, platform_id: &str, random_id: bool) -> QueryResult<Account> {
        use schema::*;

        let id: i32;
        if random_id {
            id = thread_rng().gen();
        } else {
            id = accounts::table
                .order(accounts::id.desc())
                .select(accounts::id)
                .first(&self.0)
                .optional()?
                .unwrap_or(0)
                + 1;
        }

        let id_str = id.to_string();

        insert_into(accounts::table)
            .values((accounts::id.eq(id), accounts::username.eq(&id_str), accounts::displayname.eq(&id_str)))
            .execute(&self.0)?;
        
        let result: Account = self.get_account(id)?.unwrap();

        // link account to platform id
        insert_into(platform_id_links::table)
            .values((platform_id_links::platform.eq(platform), platform_id_links::platform_id.eq(platform_id), platform_id_links::account_id.eq(result.id)))
            .execute(&self.0)?;
            
        Ok(result)
    }

    pub fn set_display_name(&self, account_id: i32, value: &str) -> QueryResult<()> {
        use schema::accounts::dsl::*;

        diesel::update(accounts.filter(id.eq(account_id)))
            .set(displayname.eq(value))
            .execute(&self.0)?;

        Ok(())
    }

    pub fn get_current_avatar(&self, account_id: i32) -> QueryResult<Option<String>> {
        use schema::accounts::dsl::*;

        accounts.find(account_id)
            .select(current_avatar)
            .first(&self.0)
    }

    pub fn set_current_avatar(&self, account_id: i32, value: &str) -> QueryResult<()> {
        use schema::accounts::dsl::*;

        diesel::update(accounts.filter(id.eq(account_id)))
            .set(current_avatar.eq(value))
            .execute(&self.0)
            .map(|_| ())
    }
}