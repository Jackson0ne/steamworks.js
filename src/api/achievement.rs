use napi_derive::napi;

#[napi]
pub mod achievement {
    #[napi]
    pub fn activate(achievement: String) -> bool {
        let client = crate::client::get_client();
        client
            .user_stats()
            .achievement(&achievement)
            .set()
            .and_then(|_| client.user_stats().store_stats())
            .is_ok()
    }

    #[napi]
    pub fn is_activated(achievement: String) -> bool {
        let client = crate::client::get_client();
        client
            .user_stats()
            .achievement(&achievement)
            .get()
            .unwrap_or(false)
    }

    #[napi]
    pub fn clear(achievement: String) -> bool {
        let client = crate::client::get_client();
        client
            .user_stats()
            .achievement(&achievement)
            .clear()
            .and_then(|_| client.user_stats().store_stats())
            .is_ok()
    }

    #[napi]
    pub fn get_achievement_display_attribute(achievement: String, key: String) -> String {
        let client = crate::client::get_client();
        client
            .user_stats()
            .achievement(&achievement)
            .get_achievement_display_attribute(&key)
            .expect(&format!("Error getting \"{}\" attribute for \"{}\"",&key,&achievement))
            .to_string()
    }

    #[napi]
    pub fn get_achievement_achieved_percent(achievement: String) -> f32 {
        let client = crate::client::get_client();
        client
            .user_stats()
            .achievement(&achievement)
            .get_achievement_achieved_percent()
            .expect(&format!("Failed to fetch achievement percentage for {}",&achievement))
    }

    #[napi]
    pub fn get_achievement_icon(achievement: String) -> Option<Vec<u8>> {
        let client = crate::client::get_client();
        Some(client
            .user_stats()
            .achievement(&achievement)
            .get_achievement_icon()
            .expect(&format!("Failed to get achievement icon for {}",&achievement))
        )
    }
}
